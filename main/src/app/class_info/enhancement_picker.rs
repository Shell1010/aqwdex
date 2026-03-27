use yew::prelude::*;
use backend::gear::{Enhancement, EnhancementPattern};

#[derive(Properties, PartialEq)]
pub struct PickerProps {
    pub label: String,
    pub enhancement: Enhancement,
    pub on_change: Callback<Enhancement>,
}

#[function_component(EnhancementPicker)]
pub fn enhancement_picker(props: &PickerProps) -> Html {
    let on_input = {
        let props_on_change = props.on_change.clone();
        let current = props.enhancement.clone();
        
        Callback::from(move |(lvl, rar, pat): (Option<u32>, Option<u32>, Option<EnhancementPattern>)| {
            let mut new_val = current.clone();
            if let Some(l) = lvl { new_val.level = l; }
            if let Some(r) = rar { new_val.rarity = r; }
            if let Some(p) = pat { new_val.pattern = p; }
            props_on_change.emit(new_val);
        })
    };

    html! {
        <div class="picker-group">
            <h4>{ &props.label }</h4>
            <label>{"Level"}</label>
            <input type="number" value={props.enhancement.level.to_string()} 
                oninput={
                    let on_input = on_input.clone();
                    Callback::from(move |e: InputEvent| {
                        let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok();
                        
                        on_input.emit((val, None, None))
                    })
                } 
            />

            <label>{"Rarity"}</label>
            <input type="number" value={props.enhancement.rarity.to_string()} 
                oninput={
                    let on_input = on_input.clone();
                    Callback::from(move |e: InputEvent| {
                        let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok();
                        on_input.emit((None, val, None))
                    })
                } 
            />

            <label>{"Pattern"}</label>
            // Adventurer,
            // Fighter,
            // Thief,
            // Armsman,
            // Hybrid,
            // Wizard,
            // Healer,
            // Spellbreaker,
            // Lucky,
            // Forge,
            // Vim,
            // Hearty,
            // Examen,
            // #[default] Pneuma,
            // Anima,
            <select onchange={
                let on_input = on_input.clone();
                Callback::from(move |e: Event| {
                    let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                    let pat = match val.as_str() {
                        "Adventurer" => Some(EnhancementPattern::Adventurer),
                        "Fighter" => Some(EnhancementPattern::Fighter),
                        "Thief" => Some(EnhancementPattern::Thief),
                        "Armsman" => Some(EnhancementPattern::Armsman),
                        "Hybrid" => Some(EnhancementPattern::Hybrid),
                        "Wizard" => Some(EnhancementPattern::Wizard),
                        "Healer" => Some(EnhancementPattern::Healer),
                        "Spellbreaker" => Some(EnhancementPattern::Spellbreaker),
                        "Lucky" => Some(EnhancementPattern::Lucky),
                        "Forge" => Some(EnhancementPattern::Forge),
                        "Vim" => Some(EnhancementPattern::Vim),
                        "Examen" => Some(EnhancementPattern::Examen),
                        "Hearty" => Some(EnhancementPattern::Hearty),
                        "Pneuma" => Some(EnhancementPattern::Pneuma),
                        "Anima" => Some(EnhancementPattern::Anima),
                        _ => None,
                    };
                    on_input.emit((None, None, pat))
                })
            }>
                <option selected={props.enhancement.pattern == EnhancementPattern::Adventurer}>{"Adventurer"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Fighter}>{"Fighter"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Anima}>{"Anima"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Armsman}>{"Armsman"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Hybrid}>{"Hybrid"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Wizard}>{"Wizard"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Healer}>{"Healer"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Spellbreaker}>{"Spellbreaker"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Lucky}>{"Lucky"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Forge}>{"Forge"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Vim}>{"Vim"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Examen}>{"Examen"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Hearty}>{"Hearty"}</option>
                <option selected={props.enhancement.pattern == EnhancementPattern::Pneuma}>{"Pneuma"}</option>
                
            </select>
        </div>
    }
}