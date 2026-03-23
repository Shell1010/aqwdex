use std::str::FromStr;

use backend::{damage::{Weapon, WeaponBoost}, gear::{ Enhancement, EnhancementPattern, GearSlot, get_stats}, player::{Class, ClassModel, Player, PrimaryStats, SecondaryStats}};
use yew::prelude::*;
use crate::app::class_info::enhancement_picker::EnhancementPicker;
use crate::app::class_info::stats::StatDisplay;
use crate::app::class_info::skills::Skills;

#[derive(Clone, PartialEq)]
pub struct ClassSettings {
    pub level: Player,
    pub equipment: Equipment,
    pub weapon: Weapon,
    pub class: Class,
    pub primary_stats: PrimaryStats,
    pub secondary_stats: SecondaryStats,
}

#[derive(Clone, PartialEq)]
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
        total
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Equipment { 
            helm:  Enhancement { level: 100, rarity: 5, pattern: EnhancementPattern::Anima },
            cape: Enhancement { level: 100, rarity: 5, pattern: EnhancementPattern::Forge },
            weapon: Enhancement { level: 100, rarity: 5, pattern: EnhancementPattern::Fighter },
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
        let equip_stats = equipment.total_stats();
        primary_stats.add(&equip_stats);

        ClassSettings {
            level: Player::default(),
            equipment: Equipment::default(),
            weapon: Weapon { range: 1.0, dps: 85.0, boost: backend::damage::WeaponBoost::Boost51x50 },
            class: Class::default(),
            primary_stats: primary_stats.clone(),
            secondary_stats: class.class_model.secondary_stats_convert(&player, &primary_stats)
        }
    }
}

impl ClassSettings {
    pub fn refresh_stats(&mut self) {
        let mut primary = self.class.class_model.level_primary_stat_total(&self.level);
        primary.add(&self.equipment.total_stats());
        
        self.primary_stats = primary.clone();
        self.secondary_stats = self.class.class_model.secondary_stats_convert(&self.level, &primary);
    }
}


#[function_component(PlayerSettings)]
pub fn player_settings() -> Html {
    let settings = use_state(|| ClassSettings::default());

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
    
    

    html! {
        <div class="class-config">
            <h2>{"Player Configuration"}</h2>
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

            <hr />
            <Skills settings={(*settings).clone()} />
            
        </div>
    }
}