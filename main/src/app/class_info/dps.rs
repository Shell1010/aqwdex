use yew::prelude::*;
use crate::app::class_info::class::{ClassSettings, calculate_secondary_changes, calculate_primary_changes, calculate_enemy_changes, enemy_incoming_modifier};
use crate::app::class_info::passive::{CustomPassive, TargetType};
use backend::enemy::EnemySecondaryStats;

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
    let test_duration = use_state(|| 60.0_f32);
    let is_auto_attack = use_state(|| true);
    let rotation = use_state(|| vec![
        RotationAction::Skill(1),
        RotationAction::Skill(2),
        RotationAction::Skill(3),
        RotationAction::Skill(4),
    ]);

    let mut active_buffs: Vec<ActiveBuff> = Vec::new();
    let mut active_enemy_debuffs: Vec<ActiveBuff> = Vec::new();

    let get_effective_stats = |actives: &Vec<ActiveBuff>| -> backend::player::SecondaryStats {
        let mut current_primary = settings.primary_stats.clone();
        for buff in actives {
            current_primary = calculate_primary_changes(&mut current_primary, &buff.passive);
        }
        let mut current_secondary = settings.class.class_model
            .secondary_stats_convert(&settings.level, &current_primary);
        for passive in &settings.passives {
            current_secondary = calculate_secondary_changes(&mut current_secondary, passive);
        }
        for buff in actives {
            current_secondary = calculate_secondary_changes(&mut current_secondary, &buff.passive);
        }
        current_secondary
    };

    let get_effective_enemy = |debuffs: &Vec<ActiveBuff>| -> EnemySecondaryStats {
        let mut current_enemy = settings.enemy.clone();
        for debuff in debuffs {
            current_enemy = calculate_enemy_changes(&mut current_enemy, &debuff.passive);
        }
        current_enemy
    };

    let duration = *test_duration;
    let mut time: f32 = 0.0;

    let mut gcd_ready_at: f32 = 0.0;
    let mut cd_ready_at: [f32; 5] = [0.0; 5];
    let mut rot_wait_until: f32 = 0.0;

    let mut rotation_idx = 0;
    let mut total_dmg: f32 = 0.0;
    let mut cast_counts = [0; 5];

    let haste = (settings.secondary_stats.haste / 100.0).clamp(0.0, 0.50);

    let compute_avg_dmg = |s_idx: usize, secondary: &backend::player::SecondaryStats, enemy: &EnemySecondaryStats| -> f32 {
        let (skill, _, _) = &settings.skills[s_idx];
        let crit = (secondary.crit_chance / 100.0).clamp(0.0, 1.0);
        let e_mod = enemy_incoming_modifier(&skill.damage_type, enemy);
        let dmg_non_crit = skill.compute(&settings.weapon, secondary, false) * e_mod;
        let dmg_crit = skill.compute(&settings.weapon, secondary, true) * e_mod;
        (dmg_non_crit * (1.0 - crit)) + (dmg_crit * crit)
    };

    let mut safety_net = 0;
    while time < duration && safety_net < 100_000 {
        safety_net += 1;

        let mut next_aa_time = f32::INFINITY;
        if *is_auto_attack {
            next_aa_time = cd_ready_at[0].max(time);
        }

        let mut next_rot_time = f32::INFINITY;
        if !rotation.is_empty() {
            if rot_wait_until > time {
                next_rot_time = rot_wait_until;
            } else {
                match &rotation[rotation_idx] {
                    RotationAction::Skill(idx) => {
                        if *idx == 0 {
                            if *is_auto_attack {
                                next_rot_time = time;
                            } else {
                                next_rot_time = cd_ready_at[0].max(time);
                            }
                        } else {
                            next_rot_time = cd_ready_at[*idx].max(gcd_ready_at).max(time);
                        }
                    }
                    RotationAction::Delay(_) => {
                        next_rot_time = time;
                    }
                }
            }
        }

        if next_aa_time == f32::INFINITY && next_rot_time == f32::INFINITY {
            break;
        }

        let t_event = next_aa_time.min(next_rot_time);
        if t_event >= duration {
            break;
        }
        let delta_ms = (t_event - time) * 1000.0;
        time = t_event;

        active_buffs.retain_mut(|b| {
            if let Some(_) = b.passive.duration {
                b.remaining_ms -= delta_ms;
                b.remaining_ms > 0.0
            } else {
                true
            }
        });
        active_enemy_debuffs.retain_mut(|b| {
            if let Some(_) = b.passive.duration {
                b.remaining_ms -= delta_ms;
                b.remaining_ms > 0.0
            } else {
                true
            }
        });

        let current_secondary = get_effective_stats(&active_buffs);
        let current_enemy = get_effective_enemy(&active_enemy_debuffs);


        let mut action_taken = false;

        if *is_auto_attack && time == next_aa_time {
            total_dmg += compute_avg_dmg(0, &current_secondary, &current_enemy);
            cast_counts[0] += 1;
            let curr_h = (current_secondary.haste / 100.0).clamp(0.0, 0.50);
            let (aa_skill, aa_passives, _) = &settings.skills[0];
            for (p_idx, passive) in aa_passives.iter().enumerate() {
                if passive.target_type == TargetType::Enemy {
                    if let Some(existing) = active_enemy_debuffs.iter_mut().find(|b| b.skill_idx == 0 && b.passive_idx == p_idx) {
                        if let Some(d) = passive.duration { existing.remaining_ms = d as f32; }
                    } else {
                        active_enemy_debuffs.push(ActiveBuff {
                            skill_idx: 0, passive_idx: p_idx,
                            remaining_ms: passive.duration.unwrap_or(0) as f32,
                            passive: passive.clone(),
                        });
                    }
                } else {
                    if let Some(existing) = active_buffs.iter_mut().find(|b| b.skill_idx == 0 && b.passive_idx == p_idx) {
                        if let Some(d) = passive.duration {
                            existing.remaining_ms = d as f32;
                        }
                    } else {
                        active_buffs.push(ActiveBuff {
                            skill_idx: 0,
                            passive_idx: p_idx,
                            remaining_ms: passive.duration.unwrap_or(0) as f32,
                            passive: passive.clone(),
                        });
                    }
                }
            }
            cd_ready_at[0] = time + ((aa_skill.cd as f32 / 1000.0) * (1.0 - curr_h)).max(0.001);
            action_taken = true;
        }

        if !rotation.is_empty() && time == next_rot_time {
            if rot_wait_until <= time {
                match &rotation[rotation_idx] {
                    RotationAction::Skill(s) => {
                        let s_idx = *s;

                        if s_idx == 0 && *is_auto_attack {
                        } else if time >= cd_ready_at[s_idx] && (s_idx == 0 || time >= gcd_ready_at) {
                            total_dmg += compute_avg_dmg(s_idx, &current_secondary, &current_enemy);
                            cast_counts[s_idx] += 1;

                            let (_, skill_passives, _) = &settings.skills[s_idx];
                            for (p_idx, passive) in skill_passives.iter().enumerate() {
                                if passive.target_type == TargetType::Enemy {
                                    if let Some(existing) = active_enemy_debuffs.iter_mut().find(|b| b.skill_idx == s_idx && b.passive_idx == p_idx) {
                                        if let Some(d) = passive.duration { existing.remaining_ms = d as f32; }
                                    } else {
                                        active_enemy_debuffs.push(ActiveBuff {
                                            skill_idx: s_idx, passive_idx: p_idx,
                                            remaining_ms: passive.duration.unwrap_or(0) as f32,
                                            passive: passive.clone(),
                                        });
                                    }
                                } else {
                                    if let Some(existing) = active_buffs.iter_mut().find(|b| b.skill_idx == s_idx && b.passive_idx == p_idx) {
                                        if let Some(d) = passive.duration {
                                            existing.remaining_ms = d as f32;
                                        }
                                    } else {
                                        active_buffs.push(ActiveBuff {
                                            skill_idx: s_idx,
                                            passive_idx: p_idx,
                                            remaining_ms: passive.duration.unwrap_or(0) as f32,
                                            passive: passive.clone(),
                                        });
                                    }
                                }
                            }
                            let curr_h = (current_secondary.haste / 100.0).clamp(0.0, 0.50);
                            let (skill_data, _, _) = &settings.skills[s_idx];
                            cd_ready_at[s_idx] = time + ((skill_data.cd as f32 / 1000.0) * (1.0 - curr_h)).max(0.001);
                            if s_idx != 0 {
                                gcd_ready_at = time + 1.5 * (1.0 - curr_h);
                            }
                        } else {

                        }
                        rotation_idx = (rotation_idx + 1) % rotation.len();
                    }
                    RotationAction::Delay(d) => {
                        rot_wait_until = time + d;
                        rotation_idx = (rotation_idx + 1) % rotation.len();
                    }
                }
            }
            action_taken = true;
        }

        if !action_taken {
            time += 0.001;
        }
    }

    let final_dps = if duration > 0.0 { total_dmg / duration } else { 0.0 };

    html! {
        <div class="dps-calculator panel-right-section" style="margin-top: 20px;">
            <h3>{format!("DPS Simulator {}", haste)}</h3>

            <div class="stats-screen">
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
                    <div class="input-field" style="margin-top: 10px;">
                        <label>{"Auto Attack Parallel: "}</label>
                        <input type="checkbox" checked={*is_auto_attack}
                            onclick={
                                let is_auto_attack = is_auto_attack.clone();
                                Callback::from(move |_| {
                                    is_auto_attack.set(!*is_auto_attack);
                                })
                            }
                        />
                    </div>
                </div>

                <div class="stat-block" style="border-color: var(--accent);">
                    <h4>{"Simulation Results"}</h4>
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
                    <div class="stat-row" style="margin-top: 6px; border-top: 1px dashed var(--border-color); padding-top: 6px;">
                        <span class="label">{"Skill Buffs"}</span>
                        <span class="value" style="font-size: 0.75rem; color: var(--text-muted);">
                            {format!("1:[{}] 2:[{}] 3:[{}] 4:[{}] 5:[{}]",
                                settings.skills[0].1.len(),
                                settings.skills[1].1.len(),
                                settings.skills[2].1.len(),
                                settings.skills[3].1.len(),
                                settings.skills[4].1.len(),
                            )}
                        </span>
                    </div>
                </div>
            </div>

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
