use yew::prelude::*;
use backend::gear::{Enhancement, EnhancementPattern, Trait};
use crate::app::class_info::passive::{CustomPassive, OperationType, TargetType};


#[derive(Properties, PartialEq)]
pub struct PickerProps {
    pub label: String,
    pub enhancement: Enhancement,
    pub on_change: Callback<Enhancement>,
    pub on_update_passives: Callback<Vec<CustomPassive>>,
}

fn get_passives_for_trait(tra: Trait) -> Vec<CustomPassive> {
    match tra {
        Trait::Clairvoyance => vec![
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "Hit Chance".to_string(),
                value: 10.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            }
        ],
        Trait::Vainglory => vec![
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "All Out".to_string(),
                value: 15.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "Heal In".to_string(),
                value: -50.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            }
        ],
        Trait::Lament => vec![
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "Crit Chance".to_string(),
                value: 20.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "Haste".to_string(),
                value: -5.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
        ],
        Trait::Avarice => vec![
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "Haste".to_string(),
                value: 10.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "All In".to_string(),
                value: -35.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            }
        ],
        Trait::Absolution => vec![
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "Heal Out".to_string(),
                value: 50.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "Phy Out".to_string(),
                value: -20.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
        ],
        Trait::Penitence => vec![
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "All In".to_string(),
                value: 25.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "DoT Out".to_string(),
                value: -25.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
        ],
        Trait::Ether => vec![
            CustomPassive {
                target_type: TargetType::Secondary,
                stat_name: "Mana Consumption".to_string(),
                value: -33.0,
                operation_type: OperationType::Additive,
                ..Default::default()
            },
        ],
        
        _ => vec![],
    }
}

#[function_component(EnhancementPicker)]
pub fn enhancement_picker(props: &PickerProps) -> Html {
    let on_input = {
        let props_on_change = props.on_change.clone();
        let props_on_update_passives = props.on_update_passives.clone();
        
        let current = props.enhancement.clone();
        
        Callback::from(move |(lvl, rar, pat, tra): (Option<u32>, Option<u32>, Option<EnhancementPattern>, Option<Trait>)| {
            let mut new_val = current.clone();
            if let Some(l) = lvl { new_val.level = l; }
            if let Some(r) = rar { new_val.rarity = r; }
            if let Some(p) = pat { new_val.pattern = p; }
            if let Some(t) = tra { new_val.r#trait = t; let passives = get_passives_for_trait(t); props_on_update_passives.emit(passives); }
            props_on_change.emit(new_val);
        })
    };


    html! {
        <div class="picker-group">
            <h3>{ &props.label }</h3>
            <label>{"Level"}</label>
            <input type="number" value={props.enhancement.level.to_string()} 
                oninput={
                    let on_input = on_input.clone();
                    Callback::from(move |e: InputEvent| {
                        let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok();
                        
                        on_input.emit((val, None, None, None))
                    })
                } 
            />

            <label>{"Rarity"}</label>
            <input type="number" value={props.enhancement.rarity.to_string()} 
                oninput={
                    let on_input = on_input.clone();
                    Callback::from(move |e: InputEvent| {
                        let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok();
                        on_input.emit((None, val, None, None))
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
                    on_input.emit((None, None, pat, None))
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
            <label>{"Trait"}</label>
            <select onchange={
                let on_input = on_input.clone();
                Callback::from(move |e: Event| {
                    let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                    let tra = match val.as_str() {
                        "Clairvoyance" => Some(Trait::Clairvoyance),
                        "Vainglory" => Some(Trait::Vainglory),
                        "Absolution" => Some(Trait::Absolution),
                        "Penitence" => Some(Trait::Penitence),
                        "Lament" => Some(Trait::Lament),
                        "Avarice" => Some(Trait::Avarice),
                        "Ether" => Some(Trait::Ether),
                        _ => None,
                    };
                    on_input.emit((None, None, None, tra))
                })
                
            }>
                <option selected={props.enhancement.r#trait == Trait::None}>{"None"}</option>
                <option selected={props.enhancement.r#trait == Trait::Clairvoyance}>{"Clairvoyance"}</option>
                <option selected={props.enhancement.r#trait == Trait::Vainglory}>{"Vainglory"}</option>
                <option selected={props.enhancement.r#trait == Trait::Absolution}>{"Absolution"}</option>
                <option selected={props.enhancement.r#trait == Trait::Penitence}>{"Penitence"}</option>
                <option selected={props.enhancement.r#trait == Trait::Lament}>{"Lament"}</option>
                <option selected={props.enhancement.r#trait == Trait::Avarice}>{"Avarice"}</option>
                <option selected={props.enhancement.r#trait == Trait::Ether}>{"Ether"}</option>
                
            </select>
        </div>
    }
}