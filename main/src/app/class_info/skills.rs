use std::str::FromStr;
use yew::prelude::*;
use crate::app::class_info::class::{ClassSettings, enemy_incoming_modifier};
use crate::app::class_info::passive::{CustomPassive, TargetType, OperationType};
use backend::damage::{DamageSource, Skill, Type, Target};

#[derive(Properties, PartialEq)]
pub struct SkillProps {
    pub settings: ClassSettings,
    pub on_update_skills: Callback<Vec<(Skill, Vec<CustomPassive>, bool)>>,
}


#[function_component(Skills)]
pub fn skills(props: &SkillProps) -> Html {
    let settings = &props.settings;
    let on_update_parent = props.on_update_skills.clone();
    let skills_list = &settings.skills;

    let primary_options = vec!["Strength", "Intellect", "Endurance", "Dexterity", "Wisdom", "Luck"];
    let secondary_options = vec![
        "Haste", "Crit Chance", "Hit Chance", "Dodge Chance",
        "All Out", "Phy Out", "Mag Out", "Heal Out",
        "All In", "Phy In", "Mag In", "Heal In",
        "DoT In", "DoT Out", "Mana Consumption",
        "Attack Power", "Spell Power", "Crit Modifier"
    ];
    // Stat names that map to EnemySecondaryStats fields via calculate_enemy_changes
    let enemy_options = vec![
        "All In", "Phy In", "Mag In", "DoT In", "Heal In",
        "All Out", "Phy Out", "Mag Out", "DoT Out", "Heal Out",
        "Crit Chance", "Crit Modifier", "Haste", "Dodge",
    ];

    let update_skill_at = {
        let skills_list = skills_list.clone();
        let on_update = on_update_parent.clone();
        Callback::from(move |(idx, new_skill, new_passives, new_crit): (usize, Skill, Vec<CustomPassive>, bool)| {
            let mut new_list = skills_list.clone();
            if let Some(item) = new_list.get_mut(idx) {
                *item = (new_skill, new_passives, new_crit);
                on_update.emit(new_list);
            }
        })
    };


    html! {
            <div class="skill-editor panel-right-section">
                <h3>{"Skill Overrides & Calculations"}</h3>

                <table>
                    <thead>
                        <tr>
                            <th>{"#"}</th>
                            <th>{"Damage"}</th>
                            <th>{"Function"}</th>
                            <th>{"Type"}</th>
                            <th>{"Target"}</th>
                            <th>{"CD (ms)"}</th>
                            <th>{"MP"}</th>
                            <th>{"Crit?"}</th>
                            <th>{"Output"}</th>
                            <th>{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for skills_list.iter().enumerate().map(|(i, (skill, passives, is_crit))| {
                            let skill = skill.clone();
                            let passives = passives.clone();
                            let is_crit = *is_crit;
                            let up_cb = update_skill_at.clone();

                            let enemy_mod = enemy_incoming_modifier(&skill.damage_type, &settings.enemy);
                            let res = skill.compute(&settings.weapon, &settings.secondary_stats, is_crit) * enemy_mod;

                            html! {
                                <>
                                    <tr key={format!("skill-{}", i)}>
                                        <td>{ i + 1 }</td>
                                        <td>
                                            <input type="number" class="table-input" value={skill.damage.to_string()}
                                                oninput={
                                                    let s = skill.clone();
                                                    let p = passives.clone();
                                                    let up = up_cb.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let mut s = s.clone();
                                                        let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0);
                                                        s.damage = val;
                                                        up.emit((i, s.clone(), p.clone(), is_crit));
                                                    })
                                                }
                                            />
                                        </td>
                                        <td>
                                            <select onchange={
                                                let s = skill.clone();
                                                let p = passives.clone();
                                                let up = up_cb.clone();
                                                Callback::from(move |e: Event| {
                                                    let mut s = s.clone();
                                                    let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                                    if let Ok(src) = DamageSource::from_str(&val) {
                                                        s.dsrc = src;
                                                        up.emit((i, s.clone(), p.clone(), is_crit));
                                                    }
                                                })
                                            }>
                                                <option value="AP1" selected={skill.dsrc == DamageSource::AP1}>{"AP"}</option>
                                                <option value="SP1" selected={skill.dsrc == DamageSource::SP1}>{"SP"}</option>
                                                <option value="AP2" selected={skill.dsrc == DamageSource::AP2}>{"AP * Range"}</option>
                                                <option value="SP2" selected={skill.dsrc == DamageSource::SP2}>{"SP * Range"}</option>
                                                <option value="APSP1" selected={skill.dsrc == DamageSource::APSP1}>{"AP + SP"}</option>
                                                <option value="APSP2" selected={skill.dsrc == DamageSource::APSP2}>{"AP + SP * Range"}</option>
                                                <option value="cHPm" selected={skill.dsrc == DamageSource::cHPm}>{"Max HP"}</option>
                                                <option value="intHP" selected={skill.dsrc == DamageSource::intHP}>{"Current HP"}</option>
                                                <option value="intMP" selected={skill.dsrc == DamageSource::intMP}>{"Current MP"}</option>
                                            </select>
                                        </td>
                                        <td>
                                            <select onchange={
                                                let s = skill.clone();
                                                let p = passives.clone();
                                                let up = up_cb.clone();
                                                Callback::from(move |e: Event| {
                                                    let mut s = s.clone();
                                                    let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                                    if let Ok(t) = Type::from_str(&val) {
                                                        s.damage_type = t;
                                                        up.emit((i, s.clone(), p.clone(), is_crit));
                                                    }
                                                })
                                            }>
                                                <option value="Physical" selected={skill.damage_type == Type::Physical}>{"Physical"}</option>
                                                <option value="Magical" selected={skill.damage_type == Type::Magical}>{"Magical"}</option>
                                                <option value="TrueDamage" selected={skill.damage_type == Type::TrueDamage}>{"True"}</option>
                                                <option value="DamageOverTime" selected={skill.damage_type == Type::DamageOverTime}>{"DoT"}</option>
                                            </select>
                                        </td>
                                        <td>
                                            <select onchange={
                                                let s = skill.clone();
                                                let p = passives.clone();
                                                let up = up_cb.clone();
                                                Callback::from(move |e: Event| {
                                                    let mut s = s.clone();
                                                    let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                                    if let Ok(t) = Target::from_str(&val) {
                                                        s.target = t;
                                                        up.emit((i, s.clone(), p.clone(), is_crit));
                                                    }
                                                })
                                            }>
                                                <option value="Self" selected={skill.target == Target::Yourself}>{"Self"}</option>
                                                <option value="Enemy" selected={skill.target == Target::Enemy}>{"Enemy"}</option>
                                                <option value="Friendly" selected={skill.target == Target::Friendly}>{"Friendly"}</option>
                                            </select>
                                        </td>
                                        <td>
                                            <input type="number" class="table-input" value={skill.cd.to_string()}
                                                oninput={
                                                    let s = skill.clone();
                                                    let p = passives.clone();
                                                    let up = up_cb.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let mut s = s;
                                                        s.cd = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0);
                                                        up.emit((i, s.clone(), p.clone(), is_crit));
                                                    })
                                                }
                                            />
                                        </td>
                                        <td>
                                            <input type="number" class="table-input" value={skill.mp.to_string()}
                                                oninput={
                                                    let s = skill.clone();
                                                    let p = passives.clone();
                                                    let up = up_cb.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let mut s = s.clone();
                                                        s.mp = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0);
                                                        up.emit((i, s.clone(), p.clone(), is_crit));
                                                    })
                                                }
                                            />
                                        </td>

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
                                        <td class={if is_crit { "dmg-cell crit" } else { "dmg-cell" }}>
                                            { format!("{:.1}", res) }
                                        </td>
                                        <td>
                                            <button class="add-buff-mini-btn" onclick={
                                                let s = skill.clone();
                                                let p = passives.clone();
                                                let up = up_cb.clone();
                                                Callback::from(move |_| {
                                                    let mut p = p.clone();
                                                    p.push(CustomPassive::default());
                                                    up.emit((i, s.clone(), p.clone(), is_crit));
                                                })
                                            }>{"[+] Effect"}</button>
                                        </td>
                                    </tr>


                                    { for passives.iter().enumerate().map(|(p_idx, current_passive)| {
                                        let current_passive = current_passive.clone();
                                        let stat_options = match current_passive.target_type {
                                            TargetType::Primary   => &primary_options,
                                            TargetType::Secondary => &secondary_options,
                                            TargetType::Enemy     => &enemy_options,
                                        };
                                        let is_debuff = current_passive.target_type == TargetType::Enemy;

                                        html! {
                                            <tr key={format!("skill-{}-buff-{}", i, p_idx)} class="skill-buff-row">
                                                <td colspan="9">
                                                    <div class="buff-editor">
                                                        <span class="buff-prefix" style={if is_debuff { "color: #f85149;" } else { "" }}>
                                                            { if is_debuff { format!("↳ Debuff #{}", p_idx + 1) } else { format!("↳ Buff #{}", p_idx + 1) } }
                                                        </span>


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
                                                            <option value="Primary" selected={current_passive.target_type == TargetType::Primary}>{"Primary"}</option>
                                                            <option value="Secondary" selected={current_passive.target_type == TargetType::Secondary}>{"Secondary"}</option>
                                                            <option value="Enemy" selected={current_passive.target_type == TargetType::Enemy}>{"Enemy Debuff"}</option>
                                                        </select>


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
            </div>
        }
}
