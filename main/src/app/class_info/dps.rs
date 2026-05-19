use yew::prelude::*;
use crate::app::class_info::class::{ClassSettings, calculate_secondary_changes, calculate_primary_changes, calculate_enemy_changes, enemy_incoming_modifier};
use crate::app::class_info::passive::{CustomPassive, TargetType};
use backend::{damage::Target, enemy::EnemySecondaryStats};

#[derive(Clone, PartialEq)]
pub enum RotationAction {
    Skill(usize),
    Delay(f32),
}

#[derive(Properties, PartialEq)]
pub struct DpsProps {
    pub settings: ClassSettings,
}

#[derive(Clone)]
struct ActiveBuff {
    skill_idx: usize,
    passive_idx: usize,
    remaining_ms: f32,
    passive: CustomPassive
}

#[function_component(DpsCalculator)]
pub fn dps_calculator(props: &DpsProps) -> Html {
    let settings = &props.settings;
    
    // --- UI STATE ---
    let test_duration = use_state(|| 60.0_f32);
    let enemy_hp = use_state(|| settings.enemy.hp);
    let is_auto_attack = use_state(|| true);
    let rotation = use_state(|| vec![
        RotationAction::Skill(1),
        RotationAction::Skill(2),
        RotationAction::Skill(3),
        RotationAction::Skill(4),
    ]);

    // =========================================================================
    // SIMULATION ENGINE
    // =========================================================================
    // We isolate the simulation into a block so it calculates fresh on every render.
    // If you need to add DoTs, Mana, or Health, add them to the setup variables here.
    let (total_dmg, cast_counts, final_dps, running_player_hp, player_died, running_enemy_hp, enemy_defeated) = {
        // --- 1. Setup ---
        let duration = *test_duration;
        let mut time: f32 = 0.0;
        let mut total_dmg: f32 = 0.0;
        let mut cast_counts = [0; 5];
        
        let mut active_buffs: Vec<ActiveBuff> = Vec::new();
        let mut active_enemy_debuffs: Vec<ActiveBuff> = Vec::new();

        // Cooldown trackers (Absolute time in seconds when they are ready)
        let mut gcd_ready_at: f32 = 0.0;
        let mut cd_ready_at: [f32; 5] = [0.0; 5];
        let mut enemy_cd_ready_at = vec![0.0; settings.enemy_skills.len()];
        
        let mut rot_wait_until: f32 = 0.0;
        let mut rotation_idx = 0;

        // --- 2. Helper Functions ---
        // Calculates the player's stats with all current active buffs applied
        let get_effective_stats = |actives: &Vec<ActiveBuff>| -> backend::player::SecondaryStats {
            let mut current_primary = settings.primary_stats.clone();
            for buff in actives { current_primary = calculate_primary_changes(&mut current_primary, &buff.passive); }
            
            let mut current_secondary = settings.class.class_model.secondary_stats_convert(&settings.level, &current_primary);
            for passive in &settings.passives { current_secondary = calculate_secondary_changes(&mut current_secondary, passive); }
            for buff in actives { current_secondary = calculate_secondary_changes(&mut current_secondary, &buff.passive); }
            
            current_secondary
        };
        
        let initial_stats = get_effective_stats(&active_buffs);
        let mut running_player_hp = initial_stats.hp; 
        let mut player_died = false;
        
        let has_finite_hp = *enemy_hp > 0;
        let mut running_enemy_hp = *enemy_hp;
        let mut enemy_defeated = false;
        
        // Calculates the enemy's stats with all current active debuffs applied
        let get_effective_enemy = |debuffs: &Vec<ActiveBuff>| -> EnemySecondaryStats {
            let mut current_enemy = settings.enemy.clone();
            for debuff in debuffs { current_enemy = calculate_enemy_changes(&mut current_enemy, &debuff.passive); }
            current_enemy
        };

        // Calculates raw average damage for a specific skill (Crit + Non-Crit weighted)
        let compute_avg_dmg = |s_idx: usize, secondary: &backend::player::SecondaryStats, enemy: &EnemySecondaryStats| -> (f32, f32) {
            let (skill, _, _) = &settings.skills[s_idx];
            let mut player_dmg = 0.0;
            let mut enemy_dmg = 0.0;
            let mut crit = (secondary.crit_chance / 100.0).clamp(0.0, 1.0);
            if let Some(add_crit) = skill.properties.add_crit {
                crit += add_crit;
            }

            if let Some(val) = skill.properties.mana_back {
                
            }

            if let Some(func) = skill.properties.hp_back {
                player_dmg -= -func.compute(&settings.weapon, secondary);
            }
            let dmg_non_crit = skill.compute(&settings.weapon, secondary, false);
            let avg_raw = dmg_non_crit * (1.0 + crit * (secondary.crit_mod / 100.0));
    
            if skill.target == Target::Yourself {
                player_dmg = avg_raw;
                (enemy_dmg, player_dmg) // Negative value translates to healing mathematically
            } else {
                let e_mod = enemy_incoming_modifier(&skill.damage_type, enemy);
                enemy_dmg = avg_raw;
                (enemy_dmg * e_mod, player_dmg)
            }
        };

        // Helper to apply buffs/debuffs when a skill is cast
        let apply_passives = |s_idx: usize, passives: &Vec<CustomPassive>, e_debuffs: &mut Vec<ActiveBuff>, p_buffs: &mut Vec<ActiveBuff>| {
            for (p_idx, passive) in passives.iter().enumerate() {
                let mut target_list = if passive.target_type == TargetType::Enemy { e_debuffs.clone() } else { p_buffs.clone() };
                
                // Refresh duration if it already exists, otherwise add new
                if let Some(existing) = target_list.iter_mut().find(|b| b.skill_idx == s_idx && b.passive_idx == p_idx) {
                    if let Some(d) = passive.duration { existing.remaining_ms = d as f32; }
                } else {
                    target_list.push(ActiveBuff {
                        skill_idx: s_idx, passive_idx: p_idx,
                        remaining_ms: passive.duration.unwrap_or(0) as f32,
                        passive: passive.clone(),
                    });
                }
            }
        };

        // --- 3. Main Event Loop ---
        let mut safety_net = 0;
        while time < duration && safety_net < 100_000 {
            safety_net += 1;

            // Phase A: Predict the next event time
            let next_aa_time = if *is_auto_attack { cd_ready_at[0].max(time) } else { f32::INFINITY };
            let next_enemy_time = enemy_cd_ready_at.iter().copied().fold(f32::INFINITY, f32::min);
            let mut next_rot_time = f32::INFINITY;

            
            
            if !rotation.is_empty() {
                if rot_wait_until > time {
                    next_rot_time = rot_wait_until;
                } else {
                    match &rotation[rotation_idx] {
                        RotationAction::Skill(idx) => {
                            if *idx == 0 {
                                next_rot_time = if *is_auto_attack { time } else { cd_ready_at[0].max(time) };
                            } else {
                                // Skill must wait for both its own CD and the Global CD
                                next_rot_time = cd_ready_at[*idx].max(gcd_ready_at).max(time);
                            }
                        }
                        RotationAction::Delay(_) => next_rot_time = time,
                    }
                }
            }

            // If no events left, end simulation
            let t_event = next_aa_time.min(next_rot_time).min(next_enemy_time);
            if t_event >= duration || t_event == f32::INFINITY { break; }

            // Phase B: Advance time and Decay Auras
            let delta_ms = (t_event - time) * 1000.0;
            time = t_event;

            active_buffs.retain_mut(|b| { b.remaining_ms -= delta_ms; b.passive.duration.is_none() || b.remaining_ms > 0.0 });
            active_enemy_debuffs.retain_mut(|b| { b.remaining_ms -= delta_ms; b.passive.duration.is_none() || b.remaining_ms > 0.0 });

            // Recalculate stats for this exact moment in time
            let mut current_secondary = get_effective_stats(&active_buffs);
            let current_enemy = get_effective_enemy(&active_enemy_debuffs);
            let curr_h = (current_secondary.haste / 100.0).clamp(0.0, 0.50); // AQW Hard Haste Cap at 50%

            current_secondary.current_hp = running_player_hp;
            
            let mut action_taken = false;

            // Phase C: Enemy Actions (They hit first if tied)
            for (e_idx, (e_skill, e_passives, e_crit)) in settings.enemy_skills.iter().enumerate() {
                if time == enemy_cd_ready_at[e_idx] {
                    let mut inc_dmg = e_skill.damage as f32;
                    if *e_crit { inc_dmg *= 1.0 + (current_enemy.crit_mod / 100.0); }
                    
                    current_secondary.current_hp -= inc_dmg as i32;
                    apply_passives(100 + e_idx, e_passives, &mut active_enemy_debuffs, &mut active_buffs);
                    
                    enemy_cd_ready_at[e_idx] = time + (e_skill.cooldown as f32 / 1000.0).max(0.001);
                    action_taken = true;
                }
            }

            if current_secondary.current_hp <= 0 {
                player_died = true;
                running_player_hp = 0;
                break; 
            }

            // Phase D: Execute Rotation Action
            // 
            if *is_auto_attack && time == next_aa_time {
                let (e_dmg, p_dmg) = compute_avg_dmg(0, &current_secondary, &current_enemy);
                total_dmg += e_dmg;
                
                // Apply damage to enemy and check death condition
                running_enemy_hp -= e_dmg as i32;
                if has_finite_hp && running_enemy_hp <= 0 {
                    enemy_defeated = true;
                    running_enemy_hp = 0;
                    break;
                }
    
                current_secondary.current_hp -= p_dmg as i32; 
                current_secondary.current_hp = current_secondary.current_hp.min(current_secondary.hp);
    
                cast_counts[0] += 1;
                apply_passives(0, &settings.skills[0].1, &mut active_enemy_debuffs, &mut active_buffs);
                cd_ready_at[0] = time + ((settings.skills[0].0.cd as f32 / 1000.0) * (1.0 - curr_h)).max(0.001);
                action_taken = true;
            }
            if !rotation.is_empty() && time == next_rot_time {
                if rot_wait_until <= time {
                    if let RotationAction::Skill(s_idx) = rotation[rotation_idx] {
                        if !(s_idx == 0 && *is_auto_attack) && time >= cd_ready_at[s_idx] && (s_idx == 0 || time >= gcd_ready_at) {
                            let (e_dmg, p_dmg) = compute_avg_dmg(s_idx, &current_secondary, &current_enemy);
                            total_dmg += e_dmg;

                            running_enemy_hp -= e_dmg as i32;
                            if has_finite_hp && running_enemy_hp <= 0 {
                                enemy_defeated = true;
                                running_enemy_hp = 0;
                                break;
                            }
                                                    
                            current_secondary.current_hp -= p_dmg as i32;
                            current_secondary.current_hp = current_secondary.current_hp.min(current_secondary.hp);
    
                            cast_counts[s_idx] += 1;
                            apply_passives(s_idx, &settings.skills[s_idx].1, &mut active_enemy_debuffs, &mut active_buffs);
                            
                            cd_ready_at[s_idx] = time + ((settings.skills[s_idx].0.cd as f32 / 1000.0) * (1.0 - curr_h)).max(0.001);
                            if s_idx != 0 { gcd_ready_at = time + 1.5 * (1.0 - curr_h); }
                        }
                        rotation_idx = (rotation_idx + 1) % rotation.len();
                    } else if let RotationAction::Delay(d) = rotation[rotation_idx] {
                        rot_wait_until = time + d;
                        rotation_idx = (rotation_idx + 1) % rotation.len();
                    }
                }
                action_taken = true;
            }

            running_player_hp = current_secondary.current_hp;
            
            if running_player_hp <= 0 {
                player_died = true;
                break;
            }
            if !action_taken { time += 0.001; }
        }

        let final_dps = if duration > 0.0 { total_dmg / time.min(duration) } else { 0.0 };
        (total_dmg, cast_counts, final_dps, running_player_hp, player_died, running_enemy_hp, enemy_defeated)
    };


    // =========================================================================
    // UI RENDER LOOP
    // =========================================================================
    html! {
        <div class="dps-calculator panel-right-section" style="margin-top: 20px;">
            <h3>{"DPS Simulator"}</h3>

            <div class="stats-screen">
                // --- Settings Panel ---
                <div class="stat-block">
                    <h4>{"Simulation Settings"}</h4>
                    <div class="input-field">
                        <label>{"Duration (s): "}</label>
                        <input type="number" step="1.0" min="1.0" class="table-input" style="border: 1px solid var(--border-color);"
                            value={test_duration.to_string()}
                            oninput={
                                let test_duration = test_duration.clone();
                                Callback::from(move |e: InputEvent| {
                                    let val: f32 = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(60.0);
                                    test_duration.set(val.max(1.0));
                                })
                            }
                        />
                    </div>
                    <div class="input-field">
                        <label>{"Enemy HP: "}</label>
                        <input type="number" step="1.0" min="1.0" class="table-input" style="border: 1px solid var(--border-color);"
                            value={enemy_hp.to_string()}
                            oninput={
                                let enemy_hp = enemy_hp.clone();
                                Callback::from(move |e: InputEvent| {
                                    let val: i32 = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0);
                                    enemy_hp.set(val);
                                })
                            }
                        />
                    </div>
                    

                    <div class="input-field" style="margin-top: 10px;">
                        <label>{"Auto Attack Parallel: "}</label>
                        <input type="checkbox" checked={*is_auto_attack}
                            onclick={
                                let is_auto_attack = is_auto_attack.clone();
                                Callback::from(move |_| is_auto_attack.set(!*is_auto_attack))
                            }
                        />
                    </div>
                </div>

                // --- Results Panel ---
                // --- Simulation Results Output Window ---
                <div class="stat-block" style={if player_died { "border-color: #f85149;" } else if enemy_defeated { "border-color: #7ee787;" } else { "border-color: var(--accent);" }}>
                    <h4>{"Simulation Results"}</h4>
                    
                    { if player_died {
                        html! {
                            <div class="stat-row" style="margin-bottom: 10px; padding: 5px; background: rgba(248, 81, 73, 0.1); border-radius: 4px; text-align: center;">
                                <span class="value" style="color: #f85149; font-weight: bold; width: 100%;">{"❌ FAILED: PLAYER DIED"}</span>
                            </div>
                        }
                    } else if enemy_defeated {
                        html! {
                            <div class="stat-row" style="margin-bottom: 10px; padding: 5px; background: rgba(126, 231, 135, 0.1); border-radius: 4px; text-align: center;">
                                <span class="value" style="color: #7ee787; font-weight: bold; width: 100%;">{"🏆 SUCCESS: ENEMY DEFEATED"}</span>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="stat-row" style="margin-bottom: 10px; padding: 5px; background: rgba(126, 231, 135, 0.1); border-radius: 4px; text-align: center;">
                                <span class="value" style="color: #7ee787; font-weight: bold; width: 100%;">{"✅ SURVIVED"}</span>
                            </div>
                        }
                    }}
                
                    <div class="stat-row">
                        <span class="label">{"Player HP"}</span>
                        <span class="value" style={if player_died { "color: #f85149;" } else { "" }}>
                            {format!("{:.0} / {:.0}", running_player_hp, settings.class.class_model.secondary_stats_convert(&settings.level, &settings.primary_stats).hp)}
                        </span>
                    </div>
                
                    <div class="stat-row">
                        <span class="label">{"Enemy HP"}</span>
                        <span class="value">
                            { if *enemy_hp <= 0 { "∞".to_string() } else { format!("{:.0} / {}", running_enemy_hp, *enemy_hp) } }
                        </span>
                    </div>
                
                    <div class="stat-row">
                        <span class="label">{"Total Damage"}</span>
                        <span class="value">{format!("{:.0}", total_dmg)}</span>
                    </div>
                    <div class="stat-row">
                        <span class="label">{"Average DPS"}</span>
                        <span class="value" style="color: #ff7b72;">{format!("{:.1}", final_dps)}</span>
                    </div>
                    <div class="stat-row" style="margin-top: 10px; border-top: 1px dashed var(--border-color); padding-top: 10px;">
                        <span class="label">{"Skill Casts"}</span>
                        <span class="value" style="font-size: 0.75rem; color: var(--text-muted);">
                            {format!("1:[{}] 2:[{}] 3:[{}] 4:[{}] 5:[{}]", cast_counts[0], cast_counts[1], cast_counts[2], cast_counts[3], cast_counts[4])}
                        </span>
                    </div>
                </div>
            </div>

            // --- Rotation Builder ---
            // (Same as before, skipped repeating for brevity, keep your table intact!)
            <h4>{"Skill Rotation Planner"}</h4>
            <table>
                <thead>
                    <tr>
                        <th>{"Order"}</th>
                        <th>{"Action"}</th>
                        <th>{"Value"}</th>
                        <th>{""}</th>
                    </tr>
                </thead>
                <tbody>
                    { for rotation.iter().enumerate().map(|(i, action)| {
                        let rot_handle = rotation.clone();
                        html! {
                            <tr key={i}>
                                <td>{ i + 1 }</td>
                                <td>
                                    <select onchange={
                                        let r = (*rot_handle).clone();
                                        let rot_handle = rot_handle.clone();
                                        Callback::from(move |e: Event| {
                                            let mut r = r.clone();
                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                            if val == "Delay" {
                                                r[i] = RotationAction::Delay(1.0);
                                            } else {
                                                let s_idx = val.replace("Skill ", "").parse::<usize>().unwrap_or(1) - 1;
                                                r[i] = RotationAction::Skill(s_idx);
                                            }
                                            rot_handle.set(r);
                                        })
                                    }>
                                        <option value="Skill 1" selected={matches!(action, RotationAction::Skill(0))}>{"Skill 1 (AA)"}</option>
                                        <option value="Skill 2" selected={matches!(action, RotationAction::Skill(1))}>{"Skill 2"}</option>
                                        <option value="Skill 3" selected={matches!(action, RotationAction::Skill(2))}>{"Skill 3"}</option>
                                        <option value="Skill 4" selected={matches!(action, RotationAction::Skill(3))}>{"Skill 4"}</option>
                                        <option value="Skill 5" selected={matches!(action, RotationAction::Skill(4))}>{"Skill 5"}</option>
                                        <option value="Delay" selected={matches!(action, RotationAction::Delay(_))}>{"Manual Delay"}</option>
                                    </select>
                                </td>
                                <td>
                                    { match action {
                                        RotationAction::Skill(v) => html! { <span style="color: var(--text-muted); font-size: 0.8rem;">{format!("{}", settings.skills[*v].0.damage)}</span> },
                                        RotationAction::Delay(d) => html! {
                                            <div style="display: flex; align-items: center; gap: 5px;">
                                                <input type="number" step="0.1" class="table-input" style="width: 80px;" value={d.to_string()}
                                                    oninput={
                                                        let r = (*rot_handle).clone();
                                                        let rot_handle = rot_handle.clone();
                                                        Callback::from(move |e: InputEvent| {
                                                            let mut r = r.clone();
                                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0);
                                                            r[i] = RotationAction::Delay(val);
                                                            rot_handle.set(r);
                                                        })
                                                    }
                                                />
                                                <span style="color: var(--text-muted); font-size: 0.8rem;">{"sec"}</span>
                                            </div>
                                        }
                                    }}
                                </td>
                                <td>
                                    <button class="mini-delete-btn" onclick={
                                        let r = (*rot_handle).clone();
                                        let rot_handle = rot_handle.clone();
                                        Callback::from(move |_| {
                                            let mut r = r.clone();
                                            r.remove(i);
                                            rot_handle.set(r);
                                        })
                                    }>{"DEL"}</button>
                                </td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>

            <button class="add-row-btn" onclick={
                let r = (*rotation).clone();
                let rot_handle = rotation.clone();
                Callback::from(move |_| {
                    let mut r = r.clone();
                    r.push(RotationAction::Skill(1));
                    rot_handle.set(r);
                })
            }>{"＋ Add Action to Rotation"}</button>
        </div>
    }
}