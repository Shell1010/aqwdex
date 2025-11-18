use yew::prelude::*;
use web_sys::{HtmlInputElement};
use backend::gear::{Enhancement, EnhancementPattern};

#[derive(Properties, PartialEq)]
pub struct EnhCardProps {
    pub gear_type: String,
    pub on_change: Callback<(String, Enhancement)>, // (gear_type, enhancement)
}

#[function_component(EnhCard)]
pub fn enh_card(props: &EnhCardProps) -> Html {
    let level = use_state(|| 100u32);
    let rarity = use_state(|| 6u32);
    let pattern_str = use_state(|| "pneuma".to_string());

    let enhancement_types: Vec<&'static str> = vec![
        "Adventurer",
        "Fighter",
        "Thief",
        "Armsman",
        "Hybrid",
        "Wizard",
        "Healer",
        "Spellbreaker",
        "Lucky",
        "Forge",
        "Vim",
        "Hearty",
        "Examen",
        "Pneuma",
        "Anima",
    ];


    // Helper: build Enhancement object & emit
    let emit_update = {
        let level = level.clone();
        let rarity = rarity.clone();
        let pattern_str = pattern_str.clone();
        let on_change = props.on_change.clone();
        let gear_type = props.gear_type.clone();

        move || {
            if let Some(enh) = Enhancement::new(*level, *rarity, &pattern_str) {
                on_change.emit((gear_type.clone(), enh));
            }
        }
    };

    // Level change
    let on_level_change = {
        let level = level.clone();
        let emit_update = emit_update.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(val) = input.value().parse::<u32>() {
                    if (1..=100).contains(&val) {
                        level.set(val);
                        emit_update();
                    }
                }
            }
        })
    };

    // Rarity change
    let on_rarity_change = {
        let rarity = rarity.clone();
        let emit_update = emit_update.clone();
        Callback::from(move |e: Event| {
            if let Some(sel) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(val) = sel.value().parse::<u32>() {
                    rarity.set(val);
                    emit_update();
                }
            }
        })
    };

    // Pattern change
    let on_pattern_change = {
        let pattern = pattern_str.clone();
        let emit_update = emit_update.clone();
        Callback::from(move |e: Event| {
            if let Some(sel) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                pattern.set(sel.value());
                emit_update();
            }
        })
    };

    html! {
        <div class="card enh-card">
            <h3 class="card-title">{ format!("{} Enhancement", props.gear_type) }</h3>

            <div class="enh-row">
                <label class="enh-field">
                    <span>{ "Level" }</span>
                    <input type="number"
                        min="1"
                        max="100"
                        value={level.to_string()}
                        oninput={on_level_change}
                    />
                </label>

                <label class="enh-field">
                    <span>{ "Rarity" }</span>
                    <select onchange={on_rarity_change}>
                        { for (0..=6).map(|r| html! {
                            <option selected={*rarity == r}>{ r }</option>
                        }) }
                    </select>
                </label>

                <label class="enh-field full">
                    <span>{ "Type" }</span>
                    <select onchange={on_pattern_change}>
                        { for enhancement_types.iter().map(|name| html! {
                            <option selected={pattern_str.to_lowercase() == *name}>
                                { *name }
                            </option>
                        }) }
                    </select>
                </label>
            </div>
        </div>
    }
}
