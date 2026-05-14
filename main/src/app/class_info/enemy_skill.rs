use yew::prelude::*;
use crate::app::class_info::passive::{CustomPassive, TargetType, OperationType};
use backend::enemy::EnemySkill; 

#[derive(Properties, PartialEq)]
pub struct EnemySkillProps {
    pub enemy_skills: Vec<(EnemySkill, Vec<CustomPassive>, bool)>,
    pub on_update_skills: Callback<Vec<(EnemySkill, Vec<CustomPassive>, bool)>>,
}

#[function_component(EnemySkills)]
pub fn enemy_skills(props: &EnemySkillProps) -> Html {
    let on_update_parent = props.on_update_skills.clone();
    let skills_list = &props.enemy_skills;

    // Stat dropdown options
    let primary_options = vec!["Strength", "Intellect", "Endurance", "Dexterity", "Wisdom", "Luck"];
    let secondary_options = vec![
        "Haste", "Crit Chance", "Hit Chance", "Dodge Chance",
        "All Out", "Phy Out", "Mag Out", "Heal Out",
        "All In", "Phy In", "Mag In", "Heal In",
        "DoT In", "DoT Out", "Mana Consumption",
        "Attack Power", "Spell Power", "Crit Modifier"
    ];
    let enemy_options = vec![
        "All In", "Phy In", "Mag In", "DoT In", "Heal In",
        "All Out", "Phy Out", "Mag Out", "DoT Out", "Heal Out",
        "Crit Chance", "Crit Modifier", "Haste", "Dodge",
    ];

    let update_skill_at = {
        let skills_list = skills_list.clone();
        let on_update = on_update_parent.clone();
        Callback::from(move |(idx, new_skill, new_passives, new_crit): (usize, EnemySkill, Vec<CustomPassive>, bool)| {
            let mut new_list = skills_list.clone();
            if let Some(item) = new_list.get_mut(idx) {
                *item = (new_skill, new_passives, new_crit);
                on_update.emit(new_list);
            }
        })
    };

    html! {
        <div class="skill-editor panel-right-section">
            <h3>{"Enemy Attacks & Skills"}</h3>
            <table>
                <thead>
                    <tr>
                        <th>{"#"}</th>
                        <th>{"Base Damage"}</th>
                        <th>{"CD (ms)"}</th>
                        <th>{"Can Crit?"}</th>
                        <th>{"Actions"}</th>
                    </tr>
                </thead>
                <tbody>
                    { for skills_list.iter().enumerate().map(|(i, (skill, passives, is_crit))| {
                        let skill = skill.clone();
                        let passives = passives.clone();
                        let is_crit = *is_crit;
                        let up_cb = update_skill_at.clone();

                        html! {
                            <>
                                <tr key={format!("enemy-skill-{}", i)}>
                                    <td>{ i + 1 }</td>
                                    
                                    // Base Damage Input (i32)
                                    <td>
                                        <input type="number" class="table-input" value={skill.damage.to_string()}
                                            oninput={
                                                let s = skill.clone();
                                                let p = passives.clone();
                                                let up = up_cb.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let mut s = s.clone();
                                                    s.damage = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0);
                                                    up.emit((i, s, p.clone(), is_crit));
                                                })
                                            }
                                        />
                                    </td>

                                    // Cooldown Input (u32)
                                    <td>
                                        <input type="number" class="table-input" value={skill.cooldown.to_string()}
                                            oninput={
                                                let s = skill.clone();
                                                let p = passives.clone();
                                                let up = up_cb.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let mut s = s.clone();
                                                    s.cooldown = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(2000);
                                                    up.emit((i, s, p.clone(), is_crit));
                                                })
                                            }
                                        />
                                    </td>

                                    // Crit Toggle
                                    <td>
                                        <input type="checkbox" checked={is_crit}
                                            onclick={
                                                let s = skill.clone();
                                                let p = passives.clone();
                                                let up = up_cb.clone();
                                                Callback::from(move |_| {
                                                    up.emit((i, s.clone(), p.clone(), !is_crit));
                                                })
                                            }
                                        />
                                    </td>

                                    // Actions
                                    <td>
                                        <button class="add-buff-mini-btn" onclick={
                                            let s = skill.clone();
                                            let p = passives.clone();
                                            let up = up_cb.clone();
                                            Callback::from(move |_| {
                                                let mut p = p.clone();
                                                p.push(CustomPassive::default());
                                                up.emit((i, s.clone(), p, is_crit));
                                            })
                                        }>{"[+] Effect"}</button>
                                        
                                        <button class="mini-delete-btn" style="margin-left: 5px;" onclick={
                                            let new_list = skills_list.clone();
                                            let on_up = on_update_parent.clone();
                                            Callback::from(move |_| {
                                                let mut new_list = new_list.clone();
                                                new_list.remove(i);
                                                on_up.emit(new_list.clone());
                                            })
                                        }>{"DEL"}</button>
                                    </td>
                                </tr>

                                // Passives / Buffs Sub-rows
                                { for passives.iter().enumerate().map(|(p_idx, current_passive)| {
                                    let current_passive = current_passive.clone();
                                    let stat_options = match current_passive.target_type {
                                        TargetType::Primary   => &primary_options,
                                        TargetType::Secondary => &secondary_options,
                                        TargetType::Enemy     => &enemy_options,
                                    };
                                    
                                    // Semantic note: "Enemy" target from the enemy's perspective means buffing itself.
                                    // "Primary/Secondary" target from the enemy's perspective means debuffing the player.
                                    let is_player_debuff = current_passive.target_type != TargetType::Enemy;

                                    html! {
                                        <tr key={format!("enemy-skill-{}-buff-{}", i, p_idx)} class="skill-buff-row">
                                            <td colspan="5">
                                                <div class="buff-editor">
                                                    <span class="buff-prefix" style={if is_player_debuff { "color: #f85149;" } else { "" }}>
                                                        { if is_player_debuff { format!("↳ Player Debuff #{}", p_idx + 1) } else { format!("↳ Enemy Buff #{}", p_idx + 1) } }
                                                    </span>

                                                    // Target Type Select
                                                    <select onchange={
                                                        let s = skill.clone();
                                                        let p_list = passives.clone();
                                                        let up = up_cb.clone();
                                                        Callback::from(move |e: Event| {
                                                            let mut p_list = p_list.clone();
                                                            if let Some(p) = p_list.get_mut(p_idx) {
                                                                let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                                                match val.as_str() {
                                                                    "Secondary" => {
                                                                        p.target_type = TargetType::Secondary;
                                                                        p.stat_name = "Haste".to_string();
                                                                    },
                                                                    "Enemy" => {
                                                                        p.target_type = TargetType::Enemy;
                                                                        p.stat_name = "All In".to_string();
                                                                    },
                                                                    _ => {
                                                                        p.target_type = TargetType::Primary;
                                                                        p.stat_name = "Strength".to_string();
                                                                    }
                                                                }
                                                                up.emit((i, s.clone(), p_list.clone(), is_crit));
                                                            }
                                                        })
                                                    }>
                                                        <option value="Primary" selected={current_passive.target_type == TargetType::Primary}>{"Player (Primary)"}</option>
                                                        <option value="Secondary" selected={current_passive.target_type == TargetType::Secondary}>{"Player (Secondary)"}</option>
                                                        <option value="Enemy" selected={current_passive.target_type == TargetType::Enemy}>{"Self (Enemy Stat)"}</option>
                                                    </select>

                                                    // Stat Name Select
                                                    <select onchange={
                                                        let s = skill.clone();
                                                        let p_list = passives.clone();
                                                        let up = up_cb.clone();
                                                        Callback::from(move |e: Event| {
                                                            let mut p_list = p_list.clone();
                                                            if let Some(p) = p_list.get_mut(p_idx) {
                                                                p.stat_name = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                                                up.emit((i, s.clone(), p_list.clone(), is_crit));
                                                            }
                                                        })
                                                    }>
                                                        { for stat_options.iter().map(|opt| html! {
                                                            <option value={*opt} selected={&current_passive.stat_name == opt}>{opt}</option>
                                                        })}
                                                    </select>

                                                    // Operation Select
                                                    <select onchange={
                                                        let s = skill.clone();
                                                        let p_list = passives.clone();
                                                        let up = up_cb.clone();
                                                        Callback::from(move |e: Event| {
                                                            let mut p_list = p_list.clone();
                                                            if let Some(p) = p_list.get_mut(p_idx) {
                                                                let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                                                p.operation_type = match val.as_str() {
                                                                    "Multiplicative" => OperationType::Multiplicative,
                                                                    _ => OperationType::Additive,
                                                                };
                                                                up.emit((i, s.clone(), p_list.clone(), is_crit));
                                                            }
                                                        })
                                                    }>
                                                        <option value="Additive" selected={current_passive.operation_type == OperationType::Additive}>{"Add"}</option>
                                                        <option value="Multiplicative" selected={current_passive.operation_type == OperationType::Multiplicative}>{"Mult"}</option>
                                                    </select>

                                                    // Value Input
                                                    <input type="number" step="0.01" class="table-input buff-val-input" value={current_passive.value.to_string()} oninput={
                                                        let s = skill.clone();
                                                        let p_list = passives.clone();
                                                        let up = up_cb.clone();
                                                        Callback::from(move |e: InputEvent| {
                                                            let raw = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                                            if let Ok(val) = raw.parse::<f32>() {
                                                                let mut p_list = p_list.clone();
                                                                if let Some(p) = p_list.get_mut(p_idx) {
                                                                    p.value = val;
                                                                    up.emit((i, s.clone(), p_list.clone(), is_crit));
                                                                }
                                                            }
                                                        })
                                                    } />

                                                    // Duration Input
                                                    <input type="number" step="1" class="table-input buff-val-input" value={current_passive.duration.unwrap_or(1000).to_string()} oninput={
                                                        let s = skill.clone();
                                                        let p_list = passives.clone();
                                                        let up = up_cb.clone();
                                                        Callback::from(move |e: InputEvent| {
                                                            let raw = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                                            if let Ok(val) = raw.parse::<u32>() {
                                                                let mut p_list = p_list.clone();
                                                                if let Some(p) = p_list.get_mut(p_idx) {
                                                                    p.duration = Some(val);
                                                                    up.emit((i, s.clone(), p_list.clone(), is_crit));
                                                                }
                                                            }
                                                        })
                                                    } />

                                                    // Delete Buff Button
                                                    <button class="mini-delete-btn" onclick={
                                                        let s = skill.clone();
                                                        let p_list = passives.clone();
                                                        let up = up_cb.clone();
                                                        Callback::from(move |_| {
                                                            let mut p_list = p_list.clone();
                                                            p_list.remove(p_idx);
                                                            up.emit((i, s.clone(), p_list.clone(), is_crit));
                                                        })
                                                    }>{"DEL"}</button>
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                })}
                            </>
                        }
                    })}
                </tbody>
            </table>
            
            <button class="add-row-btn" onclick={
                let new_list = skills_list.clone();
                let on_up = on_update_parent.clone();
                Callback::from(move |_| {
                    let mut new_list = new_list.clone();
                    new_list.push((EnemySkill::default(), Vec::new(), false));
                    on_up.emit(new_list.clone());
                })
            }>{"＋ Add Enemy Skill"}</button>
        </div>
    }
}