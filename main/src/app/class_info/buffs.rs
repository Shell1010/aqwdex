use yew::prelude::*;
use crate::app::class_info::class::{ClassSettings};
use crate::app::class_info::passive::{CustomPassive, TargetType, OperationType};
use crate::push_passives;


#[derive(Properties, PartialEq)]
pub struct BuffProps {
    pub settings: ClassSettings,
    pub on_update_passives: Callback<Vec<CustomPassive>>,
}

pub fn get_buff_list() -> Vec<&'static str> {
    // Every single buff in the match statement
    vec![
        "None",
        "Empowerment",
        "Resurgence",
        "Clarity",
        "Depravity",
        "Clarity Cordial",
        "Dragonheart Philtre",
        "Endurance Draught",
        "Felicitous Philtre",
        "Fleet Foot Philtre",
        "Honor Potion",
        "Potent Honor Potion",
        "Body Tonic",
        "Fate Tonic",
        "Wise Tonic",
        "Mastery Tonic",
        "Might Tonic",
        "Sage Tonic",
        "Unstable Body Tonic",
        "Unstable Fate Tonic",
        "Unstable Wise Tonic",
        "Unstable Mastery Tonic",
        "Unstable Might Tonic",
        "Unstable Sage Tonic",
        "Destruction Elixir",
        "Divine Elixir",
        "Potent Battle Elixir",
        "Potent Destruction Elixir",
        "Potent Malevolence Elixir",
        "Unstable Battle Elixir",
        "Unstable Malevolence Elixir",
    ]
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
            },
            "Resurgence" => {
                push_passives!(list,
                    Secondary, "All Out" => 30.0;
                    Secondary, "All In" => 30.0;
                );
                
            },
            "Clarity" => {
                push_passives!(list,
                    Secondary, "Haste" => 30.0;
                    Secondary, "Hit Chance" => 30.0;
                    Secondary, "Crit Modifier" => 30.0;
                );
            },
            "Depravity" => {
                push_passives!(list,
                    Secondary, "All Out" => 30.0;
                    Secondary, "Crit Chance" => 30.0;
                    Secondary, "Dodge Chance" => 30.0;
                    Secondary, "Haste" => 20.0;
                    Secondary, "Crit Modifier" => 30.0;
                );
            },
            "Clarity Cordial" => {
                push_passives!(list,
                    Primary, "Wisdom" => 1.5, Multi;
                );
            },
            "Dragonheart Philtre" => {
                push_passives!(list,
                    Secondary, "Mana Consumption" => -50.0; 
                );
            },
            "Endurance Draught" => {
                push_passives!(list,
                    Primary, "Endurance" => 1.5, Multi;
                );
            },
            "Felicitous Philtre" => {
                push_passives!(list,
                    Primary, "Luck" => 1.5, Multi;
                );
            },
            "Fleet Foot Philtre" => {
                push_passives!(list,
                    Secondary, "Haste" => 15.0;
                    Secondary, "Evasion" => 50.0;
                );
            },
            "Honor Potion" => {
                push_passives!(list,
                    Secondary, "All Out" => 50.0;
                    Secondary, "Mana Consumption" => 15.0;
                );
            },
            "Potent Honor Potion" => {
                push_passives!(list,
                    Secondary, "All Out" => 75.0;
                    Secondary, "Mana Consumption" => 25.0;
                );
            },
            "Body Tonic" => {
                push_passives!(list,
                    Primary, "Endurance" => 1.2, Multi;
                );
            },
            "Fate Tonic" => {
                push_passives!(list,
                    Primary, "Luck" => 1.2, Multi;
                );
            },
            "Mastery Tonic" => {
                push_passives!(list,
                    Primary, "Dexterity" => 1.2, Multi;
                );
            },
            "Might Tonic" => {
                push_passives!(list,
                    Primary, "Strength" => 1.2, Multi;
                );
            },
            "Wise Tonic" => {
                push_passives!(list,
                    Primary, "Wisdom" => 1.2, Multi;
                );
            },
            "Sage Tonic" => {
                push_passives!(list,
                    Primary, "Intelligence" => 1.2, Multi;
                );
            },
            "Unstable Body Tonic" => {
                push_passives!(list,
                    Primary, "Endurance" => 1.22, Multi;
                    Primary, "Luck" => 0.9, Multi;
                );
            },
            "Unstable Fate Tonic" => {
                push_passives!(list,
                    Primary, "Luck" => 1.22, Multi;
                    Primary, "Endurance" => 0.9, Multi;
                );
            },
            "Unstable Mastery Tonic" => {
                push_passives!(list,
                    Primary, "Dexterity" => 1.22, Multi;
                    Primary, "Endurance" => 0.9, Multi;
                );
            },
            "Unstable Might Tonic" => {
                push_passives!(list,
                    Primary, "Strength" => 1.22, Multi;
                    Primary, "Endurance" => 0.9, Multi;
                );
            },
            "Unstable Sage Tonic" => {
                push_passives!(list,
                    Primary, "Intelligence" => 1.22, Multi;
                    Primary, "Endurance" => 0.9, Multi;
                );
            },
            "Unstable Wise Tonic" => {
                push_passives!(list,
                    Primary, "Wisdom" => 1.22, Multi;
                    Primary, "Endurance" => 0.9, Multi;
                );
            },
            "Destruction Elixir" => {
                push_passives!(list,
                    Secondary, "Crit Modifier" => 30.0;
                );
            },
            "Divine Elixir" => {
                push_passives!(list,
                    Secondary, "Heal Out" => 35.0;
                );
            },
            "Potent Battle Elixir" => {
                push_passives!(list,
                    Secondary, "Phy Out" => 25.0;
                );
            }
            "Potent Destruction Elixir" => {
                push_passives!(list,
                    Secondary, "Crit Modifier" => 50.0;
                );
            },
            "Potent Malevolence Elixir" => {
                push_passives!(list,
                    Secondary, "Mag Out" => 25.0;
                );
            },
            "Unstable Battle Elixir" => {
                push_passives!(list,
                    Secondary, "Phy Out" => 28.0;
                    Secondary, "Critical Chance" => -10.0;
                );
            },
            "Unstable Malevolence Elixir" => {
                push_passives!(list,
                    Secondary, "Mag Out" => 28.0;
                    Secondary, "Critical Chance" => -10.0;
                );
            },
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
                    {
                        get_buff_list().into_iter().map(|name| {
                            html! {
                                <option 
                                    value={name}
                                >{name}</option>
                            }
                        }).collect::<Html>()
                    }
                    
                </select>
            </div>

            <div style="font-size: var(--fs-xs); color: var(--text-muted); margin-top: 8px; padding: 0 5px; line-height: 1.4;">
                {"Selecting a preset will append permanent stat rows to your Passives table below."}
            </div>
        </div>
    }
}