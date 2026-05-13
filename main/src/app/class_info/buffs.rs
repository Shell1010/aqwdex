use yew::prelude::*;
use crate::app::class_info::class::{ClassSettings};
use crate::app::class_info::passive::{CustomPassive, TargetType, OperationType};

#[derive(Properties, PartialEq)]
pub struct BuffProps {
    pub settings: ClassSettings,
    pub on_update_passives: Callback<Vec<CustomPassive>>,
}

#[function_component(BuffManager)]
pub fn buff_manager(props: &BuffProps) -> Html {
    let current_passives = props.settings.passives.clone();
    let on_update = props.on_update_passives.clone();

    let on_add_preset = Callback::from(move |e: Event| {
        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        let val = input.value();
        let mut list = current_passives.clone();

        match val.as_str() {
            "Empowerment" => {
                let stats = ["Strength", "Intellect", "Endurance", "Dexterity", "Wisdom", "Luck"];
                for s in stats {
                    list.push(CustomPassive {
                        target_type: TargetType::Primary,
                        stat_name: s.to_string(),
                        value: 1.2,
                        operation_type: OperationType::Multiplicative,
                        duration: None,
                    });
                }
            }
            
            _ => {}
        }
        
        on_update.emit(list);
        input.set_value(""); // Reset selection to placeholder
    });

    html! {
        <div class="passive-manager" style="margin-top: 15px; border-top: 1px solid var(--border-color); padding-top: 15px;">
            <h4 style="color: var(--accent); font-size: var(--fs-md); margin-bottom: 10px;">
                {"Stat Buff Presets"}
            </h4>
            
            <div class="build-io">
                <select 
                    class="io-btn" 
                    style="width: 100%; cursor: pointer; text-align: center;" 
                    onchange={on_add_preset}
                >
                    <option value="" disabled=true selected=true>{"Select Buff"}</option>
                    <option value="Empowerment">{"Empowerment (+20% Primary Stats)"}</option>
                    
                </select>
            </div>

            <div style="font-size: var(--fs-xs); color: var(--text-muted); margin-top: 8px; padding: 0 5px; line-height: 1.4;">
                {"Selecting a preset will append permanent stat rows to your Passives table below."}
            </div>
        </div>
    }
}