use yew::prelude::*;

use super::enh_card::EnhCard;
use std::str::FromStr;
use backend::player::ClassModel;
use backend::damage::{Weapon, WeaponBoost};

#[derive(Properties, PartialEq)]
pub struct PlayerCardProps {
    pub on_change: Callback<PlayerData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerData {
    pub level: u32,
    pub class_model: ClassModel,
    pub weapon: Weapon,
}



#[function_component(PlayerCard)]
pub fn player_card(props: &PlayerCardProps) -> Html {
    // --- state ---
    let level = use_state(|| 100_u32);
    let class_model = use_state(|| ClassModel::TankMelee);
    let weapon_range = use_state(|| 1.0_f32);
    let weapon_dps = use_state(|| 85.0_f32);
    let weapon_boost = use_state(|| WeaponBoost::Boost30);

    // --- helper to emit updated data ---
    let emit_change = {
        let level = level.clone();
        let class_model = class_model.clone();
        let weapon_range = weapon_range.clone();
        let weapon_dps = weapon_dps.clone();
        let weapon_boost = weapon_boost.clone();

        let cb = props.on_change.clone();

        Callback::from(move |_| {
            cb.emit(PlayerData {
                level: *level,
                class_model: *class_model,
                weapon: Weapon {
                    range: *weapon_range,
                    dps: *weapon_dps,
                    boost: *weapon_boost,
                }
            });
        })
    };

    // input handlers (consistent with EnhCard)
    let on_level = {
        let level = level.clone();
        let emit_change = emit_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Ok(v) = e.target_unchecked_into::<web_sys::HtmlInputElement>()
                .value()
                .parse() 
            {
                level.set(v);
                emit_change.emit(());
            }
        })
    };

    let on_class = {
        let class_model = class_model.clone();
        let emit_change = emit_change.clone();
        Callback::from(move |e: Event| {
            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            if let Ok(c) = ClassModel::from_str(&val) {
                class_model.set(c);
                emit_change.emit(());
            }
        })
    };

    let on_range = {
        let weapon_range = weapon_range.clone();
        let emit_change = emit_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Ok(v) = e.target_unchecked_into::<web_sys::HtmlInputElement>()
                .value()
                .parse()
            {
                weapon_range.set(v);
                emit_change.emit(());
            }
        })
    };

    let on_dps = {
        let weapon_dps = weapon_dps.clone();
        let emit_change = emit_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Ok(v) = e.target_unchecked_into::<web_sys::HtmlInputElement>()
                .value()
                .parse()
            {
                weapon_dps.set(v);
                emit_change.emit(());
            }
        })
    };

    let on_boost = {
        let weapon_boost = weapon_boost.clone();
        let emit_change = emit_change.clone();
        Callback::from(move |e: Event| {
            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            let new_boost = match val.as_str() {
                "15" => WeaponBoost::Boost15,
                "30" => WeaponBoost::Boost30,
                "51" => WeaponBoost::Boost51,
                "51x30" => WeaponBoost::Boost51x30,
                "51x40" => WeaponBoost::Boost51x40,
                "51x50" => WeaponBoost::Boost51x50,
                "35x75" => WeaponBoost::Boost35x75,
                _ => WeaponBoost::Boost30,
            };
            weapon_boost.set(new_boost);
            emit_change.emit(());
        })
    };


    // --- Dropdown list of class models ---
    let class_list = vec![
        "TankMelee",
        "DodgeMelee",
        "PowerMelee",
        "OffensiveCaster",
        "DefensiveCaster",
        "PowerCaster",
        "LuckHybrid",
        "FullHybrid",
    ];

    html! {
        <div class="card enh-card">
            <h3 class="card-title">{ "Player Settings" }</h3>

            <div class="enh-grid">

                <label class="enh-field">
                    <span>{ "Level" }</span>
                    <input type="number"
                        min="1"
                        max="100"
                        value={level.to_string()}
                        oninput={on_level}
                    />
                </label>

                <label class="enh-field">
                    <span>{ "Weapon Range" }</span>
                    <input type="number"
                        step="0.1"
                        value={weapon_range.to_string()}
                        oninput={on_range}
                    />
                </label>

                <label class="enh-field">
                    <span>{ "Weapon DPS" }</span>
                    <input type="number"
                        step="0.1"
                        value={weapon_dps.to_string()}
                        oninput={on_dps}
                    />
                </label>

                <label class="enh-field full">
                    <span>{ "Class Model" }</span>
                    <select onchange={on_class}>
                        { for class_list.into_iter().map(|c| html! {
                            <option selected={format!("{:?}", *class_model) == c}>{ c }</option>
                        }) }
                    </select>
                </label>

                <label class="enh-field full">
                    <span>{ "Damage Boost" }</span>
                    <select onchange={on_boost}>
                        <option value="15">{ "1.15x" }</option>
                        <option value="30">{ "1.30x" }</option>
                        <option value="51">{ "1.51x" }</option>
                        <option value="51x30">{ "1.51 × 1.30" }</option>
                        <option value="51x40">{ "1.51 × 1.40" }</option>
                        <option value="51x50">{ "1.51 × 1.50" }</option>
                        <option value="35x75">{ "1.35 × 1.75" }</option>
                    </select>
                </label>

            </div>
        </div>
    }
    
}

#[function_component(PlayerPanel)]
pub fn player_panel() -> Html {

    // A callback that receives:
    // (gear_type: String, enhancement: Enhancement)
    let on_change = Callback::from(|(gear_type, enh): (String, backend::gear::Enhancement)| {
        web_sys::console::log_1(
            &format!("{} enhancement updated: {:?}", gear_type, enh).into()
        );
    });

    let player_on_change = Callback::from(|player_data: PlayerData | {
        web_sys::console::log_1(
            &format!("{:?} Player Updated", player_data).into()
        )
    });

    html! {
        <>
            
            <PlayerCard on_change={player_on_change.clone()}/>
           
            <div class="enh-row">
                <div class="enh-grid-container">
                    <EnhCard gear_type={"Helm".to_string()}
                            on_change={on_change.clone()} />

                    <EnhCard gear_type={"Cape".to_string()}
                            on_change={on_change.clone()} />

                    <EnhCard gear_type={"Weapon".to_string()}
                            on_change={on_change.clone()} />

                    <EnhCard gear_type={"Armor".to_string()}
                            on_change={on_change.clone()} />
                </div>
            </div>

            <div class="card primary-stats">
                <h3 class="card-title">{ "Primary Stats" }</h3>
                <div class="primary-column">
                    { for ["STR","END","DEX","INT","WIS","LCK"].iter().map(|s| html! {
                        <div class="stat">
                            <span class="stat-key">{ s.to_string() }</span>
                            <span class="stat-val">{ "0" }</span>
                        </div>
                    }) }
                </div>
            </div>

            <div class="card secondary-stats">
                <h3>{ "Secondary Stats" }</h3>
                <div class="secondary-column">
                    <div>
                        <div class="kv"><span>{ "PHY OUT" }</span><span class="mono">{ "1.00" }</span></div>
                        <div class="kv"><span>{ "PHY IN" }</span><span class="mono">{ "1.00" }</span></div>
                        <div class="kv"><span>{ "MAG OUT" }</span><span class="mono">{ "1.00" }</span></div>
                        <div class="kv"><span>{ "MAG IN" }</span><span class="mono">{ "1.00" }</span></div>
                    </div>
                    <div>
                        <div class="kv"><span>{ "CRIT CHANCE" }</span><span class="mono">{ "0.15" }</span></div>
                        <div class="kv"><span>{ "CRIT MOD" }</span><span class="mono">{ "1.50" }</span></div>
                    </div>
                </div>
            </div>
        </>
    }
}
