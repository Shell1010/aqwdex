use yew::prelude::*;
use crate::app::class_info::class::ClassSettings;

#[derive(Clone, PartialEq)]
pub enum TargetType {
    Primary,
    Secondary,
}

#[derive(Clone, PartialEq)]
pub enum OperationType {
    Multiplicative,
    Additive,
}

#[derive(Clone, PartialEq)]
pub struct CustomPassive {
    pub target_type: TargetType,
    pub stat_name: String,
    pub value: f32,
    pub operation_type: OperationType,
}

impl Default for CustomPassive {
    fn default() -> Self {
        CustomPassive {
            target_type: TargetType::Primary,
            stat_name: "Strength".to_string(),
            value: 0.0,
            operation_type: OperationType::Additive,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PassiveProps {
    pub settings: ClassSettings,
    pub on_update_passives: Callback<Vec<CustomPassive>>,
}
#[function_component(PassiveManager)]
pub fn passive_manager(props: &PassiveProps) -> Html {
    let passives_state = use_state(|| Vec::<CustomPassive>::new());
    
    let primary_options = vec!["Strength", "Intellect", "Endurance", "Dexterity", "Wisdom", "Luck"];
    let secondary_options = vec![
        "Haste", "Crit Chance", "Hit Chance", "Dodge Chance", 
        "All Out", "Phy Out", "Mag Out", "Heal Out",
        "All In", "Phy In", "Mag In", "Heal In",
        "DoT In", "Dot Out", "Mana Consumption",
        "Attack Power", "Spell Power", "Crit Modifier"
    ];

    let state_handle = passives_state.clone();
    let on_update_parent = props.on_update_passives.clone();
    
    let update_at_index = Callback::from(move |(idx, updated_p): (usize, CustomPassive)| {
        let mut list = (*state_handle).clone();
        if let Some(item) = list.get_mut(idx) {
            *item = updated_p;
            on_update_parent.emit(list.clone());
            state_handle.set(list);
        }
    });

    html! {
        <div class="passive-manager">
            <h4>{"Custom Passives Workbench"}</h4>
            <table class="passive-table">
                <thead>
                    <tr>
                        <th>{"Type"}</th>
                        <th>{"Stat Name"}</th>
                        <th>{"Operation"}</th>
                        <th>{"Value"}</th>
                        <th>{"Action"}</th>
                    </tr>
                </thead>
                <tbody>
                { for (*passives_state).iter().enumerate().map(|(i, passive)| {
                    let current_passive = passive.clone();
                    let up_cb = update_at_index.clone();
                
                    let stat_options = if current_passive.target_type == TargetType::Primary { 
                        &primary_options 
                    } else { 
                        &secondary_options 
                    };
                
                    html! {
                        <tr key={i}>
                            <td>
                                <select onchange={
                                    let current_passive = current_passive.clone();
                                    let up = up_cb.clone();
                                    Callback::from(move |e: Event| {
                                        let mut p = current_passive.clone();
                                        
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        match input.value().as_str() {
                                            "Secondary" => {
                                                p.target_type = TargetType::Secondary;
                                                p.stat_name = "Haste".to_string();
                                            }
                                            _ => {
                                                p.target_type = TargetType::Primary;
                                                p.stat_name = "Strength".to_string();
                                            }
                                        }
                                        up.emit((i, p));
                                    })
                                }>
                                    <option value="Primary" selected={current_passive.target_type == TargetType::Primary}>{"Primary"}</option>
                                    <option value="Secondary" selected={current_passive.target_type == TargetType::Secondary}>{"Secondary"}</option>
                                </select>
                            </td>
                
                            <td>
                                <select onchange={
                                    let current_passive = current_passive.clone();
                                    let up = up_cb.clone();
                                    Callback::from(move |e: Event| {
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        let mut p = current_passive.clone();
                                        p.stat_name = input.value();
                                        up.emit((i, p.clone()));
                                    })
                                }>
                                    { for stat_options.iter().map(|opt| html! {
                                        <option value={*opt} selected={&current_passive.stat_name == opt}>{opt}</option>
                                    })}
                                </select>
                            </td>
                            
                            
                            
                            <td>
                                <select onchange={
                                    let current_passive = passive.clone();
                                    let up = up_cb.clone();
                                    Callback::from(move | e: Event| {
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        let mut p = current_passive.clone();
                                        match input.value().as_str() {
                                            "Additive" => {
                                                p.operation_type = OperationType::Additive;
                                            },
                                            _ => {
                                                p.operation_type = OperationType::Multiplicative;
                                                p.value = 1.0;
                                            }
                                        }
                                        up.emit((i, p));
                                    })
                                
                                }>
                                    <option value="Additive" selected={current_passive.operation_type == OperationType::Additive}>{"Additive"}</option>
                                    <option value="Multiplicative" selected={current_passive.operation_type == OperationType::Multiplicative}>{"Multiplicative"}</option>
                                </select>
                            </td>
                            
                            <td>
                                <input type="number" class="table-input" value={current_passive.value.to_string()} oninput={
                                    let current_passive = passive.clone();
                                    let up = up_cb.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let mut p = current_passive.clone();
                                        let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0);
                                        p.value = val;
                                        up.emit((i, p))
                                    })
                                } />
                            </td>
                            
                  
                            
                            <td>
                                <button class="delete-btn" onclick={
                                    let state_handle = passives_state.clone();
                                    let on_update = props.on_update_passives.clone();
                                    Callback::from(move |_| {
                                        let mut list = (*state_handle).clone();
                                        list.remove(i);
                                        on_update.emit(list.clone());
                                        state_handle.set(list);
                                    })
                                }>{"Delete"}</button>
                            </td>
                        </tr>
                    }
                })}
                </tbody>
            </table>

            <button class="add-row-btn" onclick={
                let state_h = passives_state.clone();
                Callback::from(move |_| {
                    let mut list = (*state_h).clone();
                    list.push(CustomPassive::default());
                    state_h.set(list);
                })
            }>{"＋ Add New Passive Row"}</button>
        </div>
    }
}