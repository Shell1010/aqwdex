use std::str::FromStr;
use yew::prelude::*;
use crate::app::class_info::class::ClassSettings;
use backend::damage::{DamageSource, Skill, Type};
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct SkillProps {
    pub settings: ClassSettings,
    pub on_update_skills: Callback<Vec<(Skill, bool)>>, 
}


#[function_component(Skills)]
pub fn skills(props: &SkillProps) -> Html {
    let settings = &props.settings;
    let on_update_parent = props.on_update_skills.clone(); 
    let skills_list = &settings.skills; 
    
    let update_skill_at = {
        let skills_list = skills_list.clone();
        let on_update = on_update_parent.clone();
        Callback::from(move |(idx, new_skill, new_crit): (usize, Skill, bool)| {
            let mut new_list = skills_list.clone();
            if let Some(item) = new_list.get_mut(idx) {
                *item = (new_skill, new_crit);
                on_update.emit(new_list);
            }
        })
    };

    html! {
        <div class="skill-editor">
            <h3>{"Skill Overrides & Calculations"}</h3>
            <table>
                <thead>
                    <tr>
                        <th>{"#"}</th>
                        <th>{"Damage"}</th>
                        <th>{"DSRC"}</th>
                        <th>{"Type"}</th>
                        <th>{"CD (ms)"}</th>
                        <th>{"MP"}</th>
                        <th>{"Crit?"}</th>
                        <th>{"Output"}</th>
                    </tr>
                </thead>
                <tbody>
                    { for skills_list.iter().enumerate().map(|(i, (skill, is_crit))| {
                        let skill = skill.clone();
                        let is_crit = *is_crit;
                        let up_cb = update_skill_at.clone();

                        let res = skill.compute(&settings.weapon, &settings.secondary_stats, is_crit);

                        html! {
                            <tr key={i}>
                                <td>{ i + 1 }</td>
                                <td>
                                    <input type="number" class="table-input" value={skill.damage.to_string()} 
                                        oninput={
                                            let s = skill.clone();
                                            let up = up_cb.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let mut s = s.clone();
                                                let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0);
                                                s.damage = val;
                                                up.emit((i, s.clone(), is_crit));
                                            })
                                        } 
                                    />
                                </td>

                                <td>
                                    <select onchange={
                                        let s = skill.clone();
                                        let up = up_cb.clone();
                                        
                                        Callback::from(move |e: Event| {
                                            let mut s = s.clone();
                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                            if let Ok(src) = DamageSource::from_str(&val) {
                                                s.dsrc = src;
                                                up.emit((i, s.clone(), is_crit));
                                            }
                                        })
                                    }>
                                    <option value="AP1" selected={skill.dsrc == DamageSource::AP1}>{"AP1"}</option>
                                    <option value="SP1" selected={skill.dsrc == DamageSource::SP1}>{"SP1"}</option>
                                    <option value="AP2" selected={skill.dsrc == DamageSource::AP2}>{"AP2"}</option>
                                    <option value="SP2" selected={skill.dsrc == DamageSource::SP2}>{"SP2"}</option>
                                    <option value="APSP1" selected={skill.dsrc == DamageSource::APSP1}>{"APSP1"}</option>
                                    <option value="APSP2" selected={skill.dsrc == DamageSource::APSP2}>{"APSP2"}</option>
                                    <option value="cHPm" selected={skill.dsrc == DamageSource::cHPm}>{"cHPm"}</option>
                                    <option value="intHP" selected={skill.dsrc == DamageSource::intHP}>{"intHP"}</option>
                                    <option value="intMP" selected={skill.dsrc == DamageSource::intMP}>{"intMP"}</option>
                                    </select>
                                </td>

                                <td>
                                    <select onchange={
                                        let s = skill.clone();
                                        let up = up_cb.clone();
                                        Callback::from(move |e: Event| {
                                            let mut s = s.clone();
                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                            if let Ok(t) = Type::from_str(&val) {
                                                s.damage_type = t;
                                                up.emit((i, s.clone(), is_crit));
                                            }
                                        })
                                    }>
                                        <option value="Physical" selected={skill.damage_type == Type::Physical}>{"Physical"}</option>
                                        <option value="Magical" selected={skill.damage_type == Type::Magical}>{"Magical"}</option>
                                        <option value="TrueDamage" selected={skill.damage_type == Type::TrueDamage}>{"True Damage"}</option>
                                        <option value="DamageOverTime" selected={skill.damage_type == Type::DamageOverTime}>{"Damage Over Time"}</option>
                                    </select>
                                </td>

                                <td>
                                    <input type="number" class="table-input" value={skill.cd.to_string()} 
                                        oninput={
                                            let s = skill.clone();
                                            let up = up_cb.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let mut s = s;
                                                s.cd = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0);
                                                up.emit((i, s.clone(), is_crit));
                                            })
                                        } 
                                    />
                                </td>

                                <td>
                                    <input type="number" class="table-input" value={skill.mp.to_string()} 
                                        oninput={
                                            let s = skill.clone();
                                            let up = up_cb.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let mut s = s.clone();
                                                s.mp = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0);
                                                up.emit((i, s.clone(), is_crit));
                                            })
                                        } 
                                    />
                                </td>

                                <td>
                                    <input type="checkbox" checked={is_crit} 
                                        onclick={
                                            let s = skill.clone();
                                            let up = up_cb.clone();
                                            Callback::from(move |_| {
                                                up.emit((i, s.clone(), !is_crit));
                                            })
                                        } 
                                    />
                                </td>

                                <td class={if is_crit { "dmg-cell crit" } else { "dmg-cell" }}>
                                    { res }
                                </td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}