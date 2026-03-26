use std::str::FromStr;
use yew::prelude::*;
use crate::app::class_info::class::ClassSettings;
use backend::damage::{DamageSource, Skill, Type};


#[derive(Properties, PartialEq)]
pub struct SkillProps {
    pub settings: ClassSettings,
}



#[function_component(Skills)]
pub fn skills(props: &SkillProps) -> Html {
    let s = &props.settings;
    let skills_state = use_state(|| vec![(Skill::default(), false); 5]);

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
                        { for (*skills_state).iter().enumerate().map(|(i, (skill, is_crit))| {
                            let state_handle = skills_state.clone();
                            let current_skill = skill.clone();
                            let current_crit = *is_crit;

                            let res = current_skill.compute(&s.weapon, &s.secondary_stats, current_crit);
                            let sk_capture = current_skill.clone();
                            let h_capture = state_handle.clone();
                            html! {
                            <tr key={i}>
                                <td>{ i + 1 }</td>
                                <td>
                                    <input type="number" class="table-input" value={current_skill.damage.to_string()} oninput={
                                        let sk_for_closure = sk_capture.clone();
                                        let h_for_closure = h_capture.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let mut sk = sk_for_closure.clone();
                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(1.0);
                                            sk.damage = val;
                                            let mut list = (*h_for_closure).clone();
                                            list[i] = (sk, current_crit);
                                            h_for_closure.set(list);
                                        })
                                    } />
                                </td>
                                <td>
                                    <select onchange={
                                        let sk_for_closure = sk_capture.clone();
                                        let h_for_closure = h_capture.clone();
                                        Callback::from(move |e: Event| {
                                            let mut sk = sk_for_closure.clone();
                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                            match DamageSource::from_str(&val) {
                                                Ok(val) => sk.dsrc = val,
                                                Err(_) => sk.dsrc = DamageSource::default()

                                            }
                                            let mut list = (*h_for_closure).clone();
                                            list[i] = (sk, current_crit);
                                            h_for_closure.set(list);
                                        })
                                    }>
                                        <option value="AP1" selected={current_skill.dsrc == DamageSource::AP1}>{"AP1"}</option>
                                        <option value="SP1" selected={current_skill.dsrc == DamageSource::SP1}>{"SP1"}</option>
                                        <option value="AP2" selected={current_skill.dsrc == DamageSource::AP2}>{"AP2"}</option>
                                        <option value="SP2" selected={current_skill.dsrc == DamageSource::SP2}>{"SP2"}</option>
                                        <option value="APSP1" selected={current_skill.dsrc == DamageSource::APSP1}>{"APSP1"}</option>
                                        <option value="APSP2" selected={current_skill.dsrc == DamageSource::APSP2}>{"APSP2"}</option>
                                        <option value="cHPm" selected={current_skill.dsrc == DamageSource::cHPm}>{"cHPm"}</option>
                                        <option value="intHP" selected={current_skill.dsrc == DamageSource::intHP}>{"intHP"}</option>
                                        <option value="intMP" selected={current_skill.dsrc == DamageSource::intMP}>{"intMP"}</option>
                                    </select>
                                </td>


                                <td>
                                    <select onchange={
                                        let sk_for_closure = sk_capture.clone();
                                        let h_for_closure = h_capture.clone();
                                        Callback::from(move |e: Event| {
                                            let mut sk = sk_for_closure.clone();
                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                            match Type::from_str(&val) {
                                                Ok(val) => sk.damage_type = val,
                                                Err(_) => sk.damage_type = Type::Physical
                                            }
                                            let mut list = (*h_for_closure).clone();
                                            list[i] = (sk, current_crit);
                                            h_for_closure.set(list);
                                        })
                                    }>
                                        <option value="Physical" selected={current_skill.damage_type == Type::Physical}>{"Physical"}</option>
                                        <option value="Magical" selected={current_skill.damage_type == Type::Magical}>{"Magical"}</option>
                                        <option value="TrueDamage" selected={current_skill.damage_type == Type::TrueDamage}>{"True Damage"}</option>
                                        <option value="DamageOverTime" selected={current_skill.damage_type == Type::DamageOverTime}>{"DoT"}</option>
                                    </select>
                                </td>

                                <td>
                                    <input type="number" class="table-input" value={current_skill.cd.to_string()} oninput={
                                        let sk_for_closure = sk_capture.clone();
                                        let h_for_closure = h_capture.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let mut sk = sk_for_closure.clone();
                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0);
                                            sk.cd = val;
                                            let mut list = (*h_for_closure).clone();
                                            list[i] = (sk, current_crit);
                                            h_for_closure.set(list);
                                        })
                                    } />
                                </td>


                                <td>
                                    <input type="number" class="table-input" value={current_skill.mp.to_string()} oninput={
                                        let sk_for_closure = sk_capture.clone();
                                        let h_for_closure = h_capture.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let mut sk = sk_for_closure.clone();
                                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0);
                                            sk.mp = val;
                                            let mut list = (*h_for_closure).clone();
                                            list[i] = (sk, current_crit);
                                            h_for_closure.set(list);
                                        })
                                    } />
                                </td>


                                <td>
                                    <input type="checkbox" checked={current_crit} onclick={
                                        let h_for_closure = h_capture.clone();
                                        let sk_for_closure = sk_capture.clone();
                                        Callback::from(move |_| {
                                            let mut list = (*h_for_closure).clone();
                                            list[i] = (sk_for_closure.clone(), !current_crit);
                                            h_for_closure.set(list);
                                        })
                                    } />
                                </td>

                                <td class={if current_crit { "dmg-cell crit" } else { "dmg-cell" }}>
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
