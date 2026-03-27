use std::str::FromStr;
use crate::app::class_info::build_manager::BuildManager;
use backend::{damage::{Weapon, WeaponBoost}, gear::{ Enhancement, EnhancementPattern, GearSlot, get_stats}, player::{Class, ClassModel, Player, PrimaryStats, SecondaryStats}};
use gloo_console::log;
use yew::prelude::*;
use crate::app::class_info::{enhancement_picker::EnhancementPicker, passive::{CustomPassive, OperationType, TargetType}};
use crate::app::class_info::stats::StatDisplay;
use crate::app::class_info::skills::Skills;
use crate::app::class_info::passive::PassiveManager;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassSettings {
    pub name: String,
    pub level: Player,
    pub equipment: Equipment,
    pub weapon: Weapon,
    pub class: Class,
    pub primary_stats: PrimaryStats,
    pub secondary_stats: SecondaryStats,
    pub passives: Vec<CustomPassive>
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub helm: Enhancement,
    pub cape: Enhancement,
    pub weapon: Enhancement,
    pub class: Enhancement,
}

impl Equipment {
    pub fn total_stats(&self) -> PrimaryStats {
        let mut total = get_stats(&self.helm, GearSlot::Helm);
        total.add(&get_stats(&self.cape, GearSlot::Cape));
        total.add(&get_stats(&self.weapon, GearSlot::Weapon));
        total.add(&get_stats(&self.class, GearSlot::Armor));
        log!(total.strength);
        total
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Equipment {
            helm:  Enhancement { level: 100, rarity: 6, pattern: EnhancementPattern::Anima },
            cape: Enhancement { level: 100, rarity: 6, pattern: EnhancementPattern::Forge },
            weapon: Enhancement { level: 100, rarity: 6, pattern: EnhancementPattern::Forge },
            class: Enhancement { level: 100, rarity: 5, pattern: EnhancementPattern::Lucky },
        }
    }
}

impl Default for ClassSettings {
    fn default() -> Self {
        let player = Player::default();
        let equipment = Equipment::default();
        let class = Class::default();
        let mut primary_stats = class.class_model.level_primary_stat_total(&player);
        log!("strength before yes", primary_stats.strength);
        log!(player.level);
        log!(class.class_model.as_str());
        let equip_stats = equipment.total_stats();
        primary_stats.add(&equip_stats);

        ClassSettings {
            name: "Archfishy".into(),
            level: player.clone(),
            equipment: equipment.clone(),
            weapon: Weapon { range: 1.0, dps: 85.0, boost: backend::damage::WeaponBoost::Boost51x50 },
            class: class.clone(),
            primary_stats: primary_stats.clone(),
            secondary_stats: class.class_model.secondary_stats_convert(&player, &primary_stats),
            passives: vec![]
        }
    }

}

impl ClassSettings {
    pub fn refresh_stats(&mut self) {
        let mut primary_stats = self.class.class_model.level_primary_stat_total(&self.level);
        primary_stats.add(&self.equipment.total_stats());
        
        for passive in &self.passives {
            match passive.target_type {
                TargetType::Primary => {
                    match passive.stat_name.as_str() {
                        "Strength" => { 
                            match passive.operation_type {
                                OperationType::Additive => primary_stats.strength += passive.value as i32,
                                OperationType::Multiplicative => {
                                    let stat = primary_stats.strength;
                                    primary_stats.strength = (stat as f32 * passive.value) as i32;
                                }
                            }
                        },
                        "Dexterity" => {
                            match passive.operation_type {
                                OperationType::Additive => primary_stats.dexterity += passive.value as i32,
                                OperationType::Multiplicative => {
                                    let stat = primary_stats.dexterity;
                                    primary_stats.dexterity = (stat as f32 * passive.value) as i32;
                                }
                            }
                        },
                        "Wisdom" => {
                            match passive.operation_type {
                                OperationType::Additive => primary_stats.wisdom += passive.value as i32,
                                OperationType::Multiplicative => primary_stats.wisdom *= {
                                    let stat = primary_stats.wisdom as f32;
                                    (stat * passive.value).round() as i32
                                }
                            }
                        },
                        "Intelligence" => {
                            match passive.operation_type {
                                OperationType::Additive => primary_stats.intellect += passive.value as i32,
                                OperationType::Multiplicative => {
                                    let stat = primary_stats.intellect as f32;
                                    
                                    primary_stats.intellect = (stat * passive.value).round() as i32;
                                },
                            }
                        },
                        "Endurance" => {
                            match passive.operation_type {
                                OperationType::Additive => primary_stats.endurance += passive.value as i32,
                                OperationType::Multiplicative => {
                                    let stat = primary_stats.endurance as f32;
                                    
                                    primary_stats.endurance = (stat * passive.value).round() as i32;
                                }
                            }
                        },
                        "Luck" => {
                            match passive.operation_type {
                                OperationType::Additive => primary_stats.luck += passive.value as i32,
                                OperationType::Multiplicative => {
                                    let stat = primary_stats.luck as f32;
                                    
                                    primary_stats.luck = (stat * passive.value).round() as i32;
                                }
                            }
                        },
                        _ => (),
                    }
                }
                _ => ()
            }
        }
        
        self.primary_stats = primary_stats;
        let mut secondary_stats = self.class.class_model.secondary_stats_convert(&self.level, &self.primary_stats);
        
        for passive in &self.passives {
            match passive.target_type {
                TargetType::Secondary => {
                    match passive.stat_name.as_str() {
                        "All Out" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.all_out += passive.value,
                                OperationType::Multiplicative => secondary_stats.all_out *= passive.value,
                            }
                        },
                        "All In" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.all_in += passive.value,
                                OperationType::Multiplicative => secondary_stats.all_in *= passive.value,
                            }
                        },
                        "Phy Out" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.phy_out += passive.value,
                                OperationType::Multiplicative => secondary_stats.phy_out *= passive.value,
                            }
                        },
                        "Phy In" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.phy_in += passive.value,
                                OperationType::Multiplicative => secondary_stats.phy_in *= passive.value,
                            }
                        },
                        "Mag Out" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.mag_out += passive.value,
                                OperationType::Multiplicative => secondary_stats.mag_out *= passive.value,
                            }
                        },
                        "Mag In" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.mag_in += passive.value,
                                OperationType::Multiplicative => secondary_stats.mag_in *= passive.value,
                            }
                        },
                        "Heal Out" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.heal_out += passive.value,
                                OperationType::Multiplicative => secondary_stats.heal_out *= passive.value,
                            }
                        },
                        "Heal In" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.heal_in += passive.value,
                                OperationType::Multiplicative => secondary_stats.heal_in *= passive.value,
                            }
                        },
                        "DoT Out" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.dot_out += passive.value,
                                OperationType::Multiplicative => secondary_stats.dot_out *= passive.value,
                            }
                        },
                        "DoT In" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.dot_in += passive.value,
                                OperationType::Multiplicative => secondary_stats.dot_in *= passive.value,
                            }
                        },
                        "Mana Consumption" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.mana_consumption += passive.value,
                                OperationType::Multiplicative => secondary_stats.mana_consumption *= passive.value,
                            }
                        },
                        "Attack Power" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.attack_power += passive.value,
                                OperationType::Multiplicative => secondary_stats.attack_power *= passive.value,
                            }
                        },
                        "Spell Power" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.spell_power += passive.value,
                                OperationType::Multiplicative => secondary_stats.spell_power *= passive.value,
                            }
                        },
                        "Hit Chance" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.hit_chance += passive.value,
                                OperationType::Multiplicative => secondary_stats.hit_chance *= passive.value,
                            }
                        },
                        "Haste" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.haste += passive.value,
                                OperationType::Multiplicative => secondary_stats.haste *= passive.value,
                            }
                        },
                        "Dodge Chance" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.dodge += passive.value,
                                OperationType::Multiplicative => secondary_stats.dodge *= passive.value,
                            }
                        },
                        "Crit Chance" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.crit_chance += passive.value,
                                OperationType::Multiplicative => secondary_stats.crit_chance *= passive.value,
                            }
                        },
                        "Crit Modifier" => {
                            match passive.operation_type {
                                OperationType::Additive => secondary_stats.crit_mod += passive.value,
                                OperationType::Multiplicative => secondary_stats.crit_mod *= passive.value,
                            }
                        },
                        
                        _ => (),
                        
                    }
                },
                _ => ()
            }
        }
        self.secondary_stats = secondary_stats;
        
    }
}


#[function_component(PlayerSettings)]
pub fn player_settings() -> Html {
    let settings = use_state(|| ClassSettings::default());

    web_sys::console::log_1(&format!("Current Stat in Rust: {:?}", settings.equipment.helm).into());
    let on_level_input = {
        let settings = settings.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<u32>() {

                let mut new_s = (*settings).clone();
                new_s.level.level = val;
                new_s.refresh_stats();
                settings.set(new_s);
            }
        })
    };

    let on_model_change = {
            let settings = settings.clone();
            Callback::from(move |e: Event| {
                let select: web_sys::HtmlInputElement = e.target_unchecked_into();
                let val = select.value();

                let mut new_s = (*settings).clone();
                match ClassModel::from_str(&val) {
                    Ok(val) => new_s.class.class_model = val,
                    Err(_) => new_s.class.class_model = ClassModel::default(),
                }
                new_s.refresh_stats();
                settings.set(new_s);
            })
        };


        let models = vec!["Tank Melee", "Dodge Melee", "Power Melee", "Offensive Caster", "Defensive Caster", "Power Caster", "Luck Hybrid", "Full Hybrid"];

    let make_callback = |slot_name: &'static str| {
        let settings = settings.clone();
        Callback::from(move |new_enh: Enhancement| {
            let mut new_s = (*settings).clone();
            match slot_name {
                "helm" => new_s.equipment.helm = new_enh,
                "cape" => new_s.equipment.cape = new_enh,
                "weapon" => new_s.equipment.weapon = new_enh,
                "class" => new_s.equipment.class = new_enh,
                _ => (),
            }
            new_s.refresh_stats();
            settings.set(new_s);
        })
    };

    let on_weapon_change = {
        let settings = settings.clone();
        Callback::from(move |(range, dps, boost): (Option<f32>, Option<f32>, Option<WeaponBoost>)| {
            let mut new_s = (*settings).clone();
            if let Some(v) = range { new_s.weapon.range = v; }
            if let Some(v) = dps { new_s.weapon.dps = v; }
            if let Some(v) = boost { new_s.weapon.boost = v; }
            new_s.refresh_stats();
            settings.set(new_s);
        })
    };
    
    let on_add_passive = {
        let settings = settings.clone();
        
        Callback::from(move | passives: Vec<CustomPassive> | {
            let mut new_s = (*settings).clone();
            new_s.passives = passives;
            new_s.refresh_stats();
            settings.set(new_s);
        
        })
        
    };
    
    let on_name_change = {
        let settings = settings.clone();
        
        Callback::from(move |e: InputEvent| {
            let mut new_s = (*settings).clone();
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let val = input.value().parse::<String>().unwrap_or("archfishy".into()); 
            new_s.name = val;
            settings.set(new_s);
            
        })
    };
    
    let load_count = use_state(|| 0);
    
    let on_load_build = {
        let settings = settings.clone();
        Callback::from(move |mut loaded_settings: ClassSettings| {
            
            loaded_settings.refresh_stats();
            settings.set(loaded_settings)
        })
    };



    html! {
        <div class="class-config" key={*load_count}>
            <h2>{"Player Configuration"}</h2>
                <div class="input-field">
                    <label>{"Build Name: "}</label>
                    <input
                        type="text"
                        value={settings.name.clone()}
                        oninput={on_name_change}
                    />
                </div>
                
                <div class="input-field">
                    <label>{"Level: "}</label>
                    <input
                        type="number"
                        min="1"
                        max="100"
                        value={settings.level.level.to_string()}
                        oninput={on_level_input}
                    />
                </div>

                <div class="input-field">
                    <label>{"Class Model: "}</label>
                    
                    <select onchange={on_model_change}>
                        { for models.into_iter().map(|m| {
                            html! { <option value={m}>{m}</option> }
                        })}
                    </select>
                </div>


                <div class="enhancement-config">
                    <EnhancementPicker
                        label="Helm"
                        enhancement={settings.equipment.helm.clone()}
                        on_change={make_callback("helm")}
                    />
                    <EnhancementPicker
                        label="Cape"
                        enhancement={settings.equipment.cape.clone()}
                        on_change={make_callback("cape")}
                    />
                    <EnhancementPicker
                        label="Weapon"
                        enhancement={settings.equipment.weapon.clone()}
                        on_change={make_callback("weapon")}
                    />
                    <EnhancementPicker
                        label="Class"
                        enhancement={settings.equipment.class.clone()}
                        on_change={make_callback("class")}
                    />
                </div>
                <div class="weapon-config">
                    <h3>{"Weapon Metadata"}</h3>

                    <div class="input-grid">
                        <label>{"Range:"}</label>
                        <input type="number" step="0.1"
                            value={settings.weapon.range.to_string()}
                            oninput={
                                let on_weapon_change = on_weapon_change.clone();
                                Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok();
                                    on_weapon_change.emit((val, None, None))
                                })
                            }
                        />

                        <label>{"Weapon DPS:"}</label>
                        <input type="number" step="0.1"
                            value={settings.weapon.dps.to_string()}
                            oninput={
                                let on_weapon_change = on_weapon_change.clone();
                                Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok();
                                    on_weapon_change.emit((None, val, None))
                                })
                            }
                        />

                        <label>{"Special Boost:"}</label>
                        <select onchange={
                            let on_weapon_change = on_weapon_change.clone();
                            Callback::from(move |e: Event| {
                                let select = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                let boost = match select.value().as_str() {
                                    "15" => WeaponBoost::Boost15,
                                    "30" => WeaponBoost::Boost30,
                                    "51" => WeaponBoost::Boost51,
                                    "51x30" => WeaponBoost::Boost51x30,
                                    "51x40" => WeaponBoost::Boost51x40,
                                    "51x51" => WeaponBoost::Boost51x50,
                                    "35x75" => WeaponBoost::Boost35x75,
                                    "custom" => WeaponBoost::Custom(1.0),
                                    _ => WeaponBoost::Custom(1.0),
                                };
                                on_weapon_change.emit((None, None, Some(boost)))
                            })
                        }>
                            <option value="none" selected={matches!(settings.weapon.boost, WeaponBoost::Custom(1.0))}>{"None"}</option>
                            <option value="15" selected={matches!(settings.weapon.boost, WeaponBoost::Boost15)}>{"15%"}</option>
                            <option value="30" selected={matches!(settings.weapon.boost, WeaponBoost::Boost30)}>{"30%"}</option>
                            <option value="51" selected={matches!(settings.weapon.boost, WeaponBoost::Boost51)}>{"51%"}</option>
                            <option value="51x30" selected={matches!(settings.weapon.boost, WeaponBoost::Boost51x30)}>{"51% + 30%"}</option>
                            <option value="51x40" selected={matches!(settings.weapon.boost, WeaponBoost::Boost51x40)}>{"51% + 40%"}</option>
                            <option value="51x51" selected={matches!(settings.weapon.boost, WeaponBoost::Boost51x50)}>{"51% + 50%"}</option>
                            <option value="35x75" selected={matches!(settings.weapon.boost, WeaponBoost::Boost35x75)}>{"35% + 75%"}</option>
                            <option value="custom" selected={matches!(settings.weapon.boost, WeaponBoost::Custom(_))}>{"Custom Value..."}</option>
                        </select>
                        {
                            if let WeaponBoost::Custom(val) = settings.weapon.boost {
                                html! {
                                    <>
                                        <label>{"Custom Multiplier:"}</label>
                                        <input type="number" step="0.01" value={val.to_string()}
                                            oninput={
                                                let on_weapon_change = on_weapon_change.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                    if let Ok(num) = input.value().parse::<f32>() {
                                                        on_weapon_change.emit((None, None, Some(WeaponBoost::Custom(num))));
                                                    }
                                                })
                                            }
                                        />
                                    </>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
            <StatDisplay settings={(*settings).clone()} />
            <PassiveManager settings={(*settings).clone()} on_update_passives={on_add_passive} />

            <hr />
            <Skills settings={(*settings).clone()} />
            
            <div class="build-panel">
                <BuildManager 
                    current_settings={(*settings).clone()} 
                    on_load_build={on_load_build} 
                />
            </div>
            

        </div>
    }
}
