use std::char::MAX;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::damage::Weapon;
use crate::error::BackendError;
use crate::gear::GearSlot;
use crate::player;

pub const MAX_LEVEL: u32 = 100;
pub const X_FACTOR: u32 = 1640;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Copy)]
pub enum ClassModel {
    TankMelee,
    DodgeMelee,
    PowerMelee,
    OffensiveCaster,
    DefensiveCaster,
    PowerCaster,
    FullHybrid,
    LuckHybrid,
}

impl FromStr for ClassModel {
    type Err = BackendError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s
            .to_lowercase()
            .replace(' ', "")
            .replace("_", "")
            .replace("-", ""); // remove spaces and lowercase everything

        match normalized.as_str() {
            "tankmelee" => Ok(ClassModel::TankMelee),
            "dodgemelee" => Ok(ClassModel::DodgeMelee),
            "powermelee" => Ok(ClassModel::PowerMelee),
            "offensivecaster" => Ok(ClassModel::OffensiveCaster),
            "defensivecaster" => Ok(ClassModel::DefensiveCaster),
            "powercaster" => Ok(ClassModel::PowerCaster),
            "luckhybrid" => Ok(ClassModel::LuckHybrid),
            "fullhybrid" => Ok(ClassModel::FullHybrid),
            _ => Err(BackendError::InvalidClassModel(s.to_string())),
        }
    }
}






impl ClassModel {

    pub fn all() -> Vec<ClassModel> {
        vec![
            ClassModel::TankMelee,
            ClassModel::DodgeMelee,
            ClassModel::PowerMelee,
            ClassModel::OffensiveCaster,
            ClassModel::DefensiveCaster,
            ClassModel::PowerCaster,
            ClassModel::FullHybrid,
            ClassModel::LuckHybrid,
        ]
    }

    pub fn level_interstat_total(&self, player: &Player) -> i32 {
        (747/99) * (player.level as i32 - 1) + 15
    }

    pub fn level_primary_stat_total(&self, player: &Player) -> PrimaryStats {
        let list = self.level_interstat_total(player);
        match self {
            ClassModel::TankMelee => PrimaryStats {
                strength: (list as f32 * 0.27) as i32,
                intellect: (list as f32 * 0.05) as i32,
                dexterity: (list as f32 * 0.22) as i32,
                endurance: (list as f32 * 0.3) as i32,
                wisdom: (list as f32 * 0.1) as i32,
                luck: (list as f32 * 0.06) as i32,
            },
            ClassModel::DodgeMelee => PrimaryStats {
                strength: (list as f32 * 0.2) as i32,
                intellect: (list as f32 * 0.05) as i32,
                dexterity: (list as f32 * 0.33) as i32,
                endurance: (list as f32 * 0.22) as i32,
                wisdom: (list as f32 * 0.1) as i32,
                luck: (list as f32 * 0.1) as i32,
            },
            ClassModel::PowerMelee => PrimaryStats {
                strength: (list as f32 * 0.3) as i32,
                intellect: (list as f32 * 0.02) as i32,
                dexterity: (list as f32 * 0.3) as i32,
                endurance: (list as f32 * 0.18) as i32,
                wisdom: (list as f32 * 0.06) as i32,
                luck: (list as f32 * 0.14) as i32,
            },
            ClassModel::OffensiveCaster => PrimaryStats {
                strength: (list as f32 * 0.06) as i32,
                intellect: (list as f32 * 0.33) as i32,
                dexterity: (list as f32 * 0.11) as i32,
                endurance: (list as f32 * 0.2) as i32,
                wisdom: (list as f32 * 0.15) as i32,
                luck: (list as f32 * 0.15) as i32,
            },
            ClassModel::DefensiveCaster => PrimaryStats {
                strength: (list as f32 * 0.08) as i32,
                intellect: (list as f32 * 0.3) as i32,
                dexterity: (list as f32 * 0.1) as i32,
                endurance: (list as f32 * 0.27) as i32,
                wisdom: (list as f32 * 0.1) as i32,
                luck: (list as f32 * 0.15) as i32,
            },
            ClassModel::PowerCaster => PrimaryStats {
                strength: (list as f32 * 0.06) as i32,
                intellect: (list as f32 * 0.28) as i32,
                dexterity: (list as f32 * 0.05) as i32,
                endurance: (list as f32 * 0.23) as i32,
                wisdom: (list as f32 * 0.28) as i32,
                luck: (list as f32 * 0.1) as i32,
            },
            ClassModel::FullHybrid => PrimaryStats {
                strength: (list as f32 * 0.24) as i32,
                intellect: (list as f32 * 0.24) as i32,
                dexterity: (list as f32 * 0.2) as i32,
                endurance: (list as f32 * 0.2) as i32,
                wisdom: (list as f32 * 0.07) as i32,
                luck: (list as f32 * 0.05) as i32,
            },
            ClassModel::LuckHybrid => PrimaryStats {
                strength: (list as f32 * 0.22) as i32,
                intellect: (list as f32 * 0.08) as i32,
                dexterity: (list as f32 * 0.21) as i32,
                endurance: (list as f32 * 0.18) as i32,
                wisdom: (list as f32 * 0.08) as i32,
                luck: (list as f32 * 0.23) as i32,
            }
            
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ClassModel::TankMelee => "Tank Melee",
            ClassModel::DodgeMelee => "Dodge Melee",
            ClassModel::PowerMelee => "Power Melee",
            ClassModel::OffensiveCaster => "Offensive Caster",
            ClassModel::DefensiveCaster => "Defensive Caster",
            ClassModel::PowerCaster => "Power Caster",
            ClassModel::FullHybrid => "Full Hybrid",
            ClassModel::LuckHybrid => "Luck Hybrid",
        }
    }

    pub fn phy_out_convert(&self) -> f32 { 0.0 }

    pub fn phy_in_convert(&self) -> f32 { 0.0 }

    pub fn heal_out_convert(&self) -> f32 { 0.0 }

    pub fn heal_in_convert(&self) -> f32 { 0.0 }

    pub fn dot_out_convert(&self) -> f32 { 0.0 }

    pub fn dot_in_convert(&self) -> f32 { 0.0 }

    pub fn all_out_convert(&self) -> f32 { 0.0 }

    pub fn all_in_convert(&self) -> f32 { 0.0 }

    pub fn mana_consumption_convert(&self) -> f32 { 0.0 }

    pub fn attack_power_convert(&self, _player: &Player, primary: &PrimaryStats) -> f32 { 
        
        match self {
            ClassModel::TankMelee => {
                let str_ap = (primary.strength * 2) as f32;
                let luk_ap = (primary.luck as f32) * 0.7;
                str_ap + luk_ap
            },
            ClassModel::DodgeMelee => {
                let str_ap = (primary.strength * 2) as f32;
                let luk_ap = (primary.luck as f32) * 0.7;
                str_ap + luk_ap
            },
            ClassModel::PowerMelee => {
                let str_ap = (primary.strength * 2) as f32;
                let luk_ap = (primary.luck as f32) * 0.7;
                str_ap + luk_ap
            },
            ClassModel::DefensiveCaster => {
                (primary.strength * 2) as f32
            },
            ClassModel::OffensiveCaster => {
                (primary.strength * 2) as f32
            },
            ClassModel::PowerCaster => {
                (primary.strength * 2) as f32
            },
            ClassModel::FullHybrid => {
                let str_ap = (primary.strength * 2) as f32;
                let luk_ap = (primary.luck as f32) * 0.7;
                str_ap + luk_ap
            },
            ClassModel::LuckHybrid => {
                let str_ap = primary.strength as f32 * 1.4;
                let luk_ap = primary.luck as f32;
                str_ap + luk_ap
            }
        }

    }

    pub fn spell_power_convert(&self, _player: &Player, primary: &PrimaryStats) -> f32 { 
        match self {
            ClassModel::TankMelee => {
                primary.intellect as f32 * 2.0
            },
            ClassModel::DodgeMelee => {
                primary.intellect as f32 * 2.0
            },
            ClassModel::PowerMelee => {
                primary.intellect as f32 * 2.0
            },
            ClassModel::DefensiveCaster => {
                let int_sp = primary.intellect as f32 * 2.0;
                let luk_sp = primary.luck as f32 * 0.7;
                int_sp + luk_sp
            },
            ClassModel::OffensiveCaster => {
                let int_sp = primary.intellect as f32 * 2.0;
                let luk_sp = primary.luck as f32 * 0.7;
                int_sp + luk_sp
            },
            ClassModel::PowerCaster => {
                let int_sp = primary.intellect as f32 * 2.0;
                let luk_sp = primary.luck as f32 * 0.7;
                int_sp + luk_sp
            },
            ClassModel::FullHybrid => {
                let int_sp = primary.intellect as f32 * 2.0;
                let luk_sp = primary.luck as f32 * 0.7;
                int_sp + luk_sp
            },
            ClassModel::LuckHybrid => {
                let int_sp = primary.intellect as f32 * 1.4;
                let luk_sp = primary.luck as f32;
                int_sp + luk_sp
            }
        }
    }

    pub fn crit_chance_convert(&self, player: &Player, primary: &PrimaryStats) -> f32 { 
        match self {
            ClassModel::TankMelee => {
                let str_crit = (primary.strength as f32) * 0.4 * player.efficiency();
                let luk_crit = (primary.luck as f32) * 0.2 * player.efficiency();
                str_crit + luk_crit
            },
            ClassModel::DodgeMelee => {
                let str_crit = (primary.strength as f32) * 0.4 * player.efficiency();
                let luk_crit = (primary.luck as f32) * 0.2 * player.efficiency();
                str_crit + luk_crit
            },
            ClassModel::PowerMelee => {
                let str_crit = (primary.strength as f32) * 0.7 * player.efficiency();
                let luk_crit = (primary.luck as f32) * 0.2 * player.efficiency();
                str_crit + luk_crit
            },
            ClassModel::DefensiveCaster => {
                let wis_crit = (primary.wisdom as f32) * 0.4 * player.efficiency();
                let luk_crit = (primary.luck as f32) * 0.2 * player.efficiency();
                wis_crit + luk_crit
            },
            ClassModel::OffensiveCaster => {
                let wis_crit = (primary.wisdom as f32) * 0.7 * player.efficiency();
                let luk_crit = (primary.luck as f32) * 0.2 * player.efficiency();
                wis_crit + luk_crit
            },
            ClassModel::PowerCaster => {
                let wis_crit = (primary.wisdom as f32) * 0.4 * player.efficiency();
                let luk_crit = (primary.luck as f32) * 0.2 * player.efficiency();
                wis_crit + luk_crit
            },
            ClassModel::FullHybrid => {
                let str_crit = (primary.strength as f32) * 0.4 * player.efficiency();
                let luk_crit = (primary.luck as f32) * 0.2 * player.efficiency();
                str_crit + luk_crit
            },
            ClassModel::LuckHybrid => {
                let str_crit = (primary.strength as f32) * 0.4 * player.efficiency();
                let wis_crit = (primary.wisdom as f32) * 0.4 * player.efficiency();
                let luk_crit = (primary.luck as f32) * 0.3 * player.efficiency();
                str_crit + wis_crit + luk_crit
            }
        }
    }

    pub fn mag_in_convert(&self, player: &Player, primary: &PrimaryStats ) -> f32 { 
        match self {
            _ => {
                let int_mag_in = (primary.intellect as f32) * -1.0 * player.efficiency();
                int_mag_in
            }
        }
    }

    pub fn hit_chance_convert(&self, player: &Player, primary: &PrimaryStats) -> f32 { 
        match self {
            ClassModel::TankMelee => {
                let dex_hit = (primary.dexterity as f32) * 0.2 * player.efficiency();
                let luk_hit = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_hit + luk_hit
            },
            ClassModel::DodgeMelee => {
                let dex_hit = (primary.dexterity as f32) * 0.2 * player.efficiency();
                let luk_hit = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_hit + luk_hit
            },
            ClassModel::PowerMelee => {
                let dex_hit = (primary.dexterity as f32) * 0.2 * player.efficiency();
                let luk_hit = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_hit + luk_hit
            },
            ClassModel::DefensiveCaster => {
                let wis_hit = (primary.wisdom as f32) * 0.2 * player.efficiency();
                let luk_hit = (primary.luck as f32) * 0.1 * player.efficiency();
                wis_hit + luk_hit
            },
            ClassModel::OffensiveCaster => {
                let wis_hit = (primary.wisdom as f32) * 0.2 * player.efficiency();
                let luk_hit = (primary.luck as f32) * 0.1 * player.efficiency();
                wis_hit + luk_hit
            },
            ClassModel::PowerCaster => {
                let wis_hit = (primary.wisdom as f32) * 0.2 * player.efficiency();
                let luk_hit = (primary.luck as f32) * 0.1 * player.efficiency();
                wis_hit + luk_hit
            },
            ClassModel::FullHybrid => {
                let dex_hit = (primary.dexterity as f32) * 0.2 * player.efficiency();
                let luk_hit = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_hit + luk_hit
            },
            ClassModel::LuckHybrid => {
                let dex_hit = (primary.dexterity as f32) * 0.2 * player.efficiency();
                let wis_hit = (primary.wisdom as f32) * 0.2 * player.efficiency();
                let luk_hit = (primary.luck as f32) *   0.1 * player.efficiency();
                dex_hit + luk_hit + wis_hit
            }
        }
    }

    pub fn haste_convert(&self, player: &Player, primary: &PrimaryStats) -> f32 { 
        match self {
            ClassModel::TankMelee => {
                let dex_haste = (primary.dexterity as f32) * 0.3 * player.efficiency();
                let luk_haste = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_haste + luk_haste
            },
            ClassModel::DodgeMelee => {
                let dex_haste = (primary.dexterity as f32) * 0.5 * player.efficiency();
                let luk_haste = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_haste + luk_haste
            },
            ClassModel::PowerMelee => {
                let dex_haste = (primary.dexterity as f32) * 0.5 * player.efficiency();
                let luk_haste = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_haste + luk_haste
            },
            ClassModel::DefensiveCaster => {
                let int_haste = (primary.intellect as f32) * 0.5 * player.efficiency();
                let luk_haste = (primary.luck as f32) * 0.1 * player.efficiency();
                int_haste + luk_haste
            },
            ClassModel::OffensiveCaster => {
                let int_haste = (primary.intellect as f32) * 0.3 * player.efficiency();
                let luk_haste = (primary.luck as f32) * 0.1 * player.efficiency();
                int_haste + luk_haste
            },
            ClassModel::PowerCaster => {
                let int_haste = (primary.intellect as f32) * 0.3 * player.efficiency();
                let luk_haste = (primary.luck as f32) * 0.1 * player.efficiency();
                int_haste + luk_haste
            },
            ClassModel::FullHybrid => {
                let int_haste = (primary.intellect as f32) * 0.3 * player.efficiency();
                let dex_haste = (primary.dexterity as f32) * 0.3 * player.efficiency();
                let luk_haste = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_haste + luk_haste + int_haste
            },
            ClassModel::LuckHybrid => {
                let int_haste = (primary.intellect as f32) * 0.3 * player.efficiency();
                let dex_haste = (primary.dexterity as f32) * 0.3 * player.efficiency();
                let luk_haste = (primary.luck as f32) * 0.3 * player.efficiency();
                dex_haste + luk_haste + int_haste
            }
        }
    }

    pub fn crit_mod_convert(&self) -> f32 { 
        match self {
            ClassModel::LuckHybrid => 2.5,
            _ => 5.0,
        }
        
    }
    

    pub fn dodge_convert(&self, player: &Player, primary: &PrimaryStats) -> f32 { 
        match self {
            ClassModel::TankMelee => {
                let dex_dodge = (primary.dexterity as f32) * 0.3 * player.efficiency();
                let wis_dodge = (primary.wisdom as f32) * 0.3 * player.efficiency();
                let luk_dodge = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_dodge + luk_dodge + wis_dodge
            },
            ClassModel::DodgeMelee => {
                let dex_dodge = (primary.dexterity as f32) * 0.5 * player.efficiency();
                let wis_dodge = (primary.wisdom as f32) * 0.3 * player.efficiency();
                let luk_dodge = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_dodge + luk_dodge + wis_dodge
            },
            ClassModel::PowerMelee => {
                let dex_dodge = (primary.dexterity as f32) * 0.5 * player.efficiency();
                let wis_dodge = (primary.wisdom as f32) * 0.3 * player.efficiency();
                let luk_dodge = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_dodge + luk_dodge + wis_dodge
            },
            ClassModel::DefensiveCaster => {
                let dex_dodge = (primary.dexterity as f32) * 0.3 * player.efficiency();
                let wis_dodge = (primary.wisdom as f32) * 0.3 * player.efficiency();
                let luk_dodge = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_dodge + luk_dodge + wis_dodge
            },
            ClassModel::OffensiveCaster => {
                let dex_dodge = (primary.dexterity as f32) * 0.3 * player.efficiency();
                let wis_dodge = (primary.wisdom as f32) * 0.3 * player.efficiency();
                let luk_dodge = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_dodge + luk_dodge + wis_dodge
            },
            ClassModel::PowerCaster => {
                let dex_dodge = (primary.dexterity as f32) * 0.3 * player.efficiency();
                let wis_dodge = (primary.wisdom as f32) * 0.3 * player.efficiency();
                let luk_dodge = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_dodge + luk_dodge + wis_dodge
            },
            ClassModel::FullHybrid => {
                let dex_dodge = (primary.dexterity as f32) * 0.5 * player.efficiency();
                let wis_dodge = (primary.wisdom as f32) * 0.3 * player.efficiency();
                let luk_dodge = (primary.luck as f32) * 0.1 * player.efficiency();
                dex_dodge + luk_dodge + wis_dodge
            },
            ClassModel::LuckHybrid => {
                let dex_dodge = (primary.dexterity as f32) * 0.3 * player.efficiency();
                let wis_dodge = (primary.wisdom as f32) * 0.3 * player.efficiency();
                let luk_dodge = (primary.luck as f32) * 0.25 * player.efficiency();
                dex_dodge + luk_dodge + wis_dodge
            }
        }
    }
    fn mag_out_convert(&self, player: &Player, primary: &PrimaryStats ) -> f32 { 
        match self {
            ClassModel::DefensiveCaster | ClassModel::OffensiveCaster | &ClassModel::PowerCaster | ClassModel::FullHybrid => {
                let int_mag_out = (primary.intellect as f32) * 1.0 * player.efficiency();
                int_mag_out
            },
            _ => 0.0,
        }
    }
    pub fn secondary_stats_convert(&self, player: &Player, primary: &PrimaryStats) -> SecondaryStats {
        let hp = player.base_hp() + (primary.endurance as i32 * 5) as i32;
        SecondaryStats {
            phy_out: self.phy_out_convert(),
            phy_in: self.phy_in_convert(),
            mag_out: self.mag_out_convert(player, primary),
            mag_in: self.mag_in_convert(player, primary),
            dot_out: self.dot_out_convert(),
            dot_in: self.dot_in_convert(),
            heal_out: self.heal_out_convert(),
            heal_in: self.heal_in_convert(),
            all_in: self.all_in_convert(),
            all_out: self.all_out_convert(),
            crit_chance: self.crit_chance_convert(player, primary),
            crit_mod: self.crit_mod_convert(),
            mana_consumption: self.mana_consumption_convert(),
            haste: self.haste_convert(player, primary),
            dodge: self.dodge_convert(player, primary),
            hit_chance: self.hit_chance_convert(player, primary),
            attack_power: self.attack_power_convert(player, primary),
            spell_power: self.spell_power_convert(player, primary),
            hp: player.base_hp() + (primary.endurance as i32 * 5) as i32,
            current_hp: hp,
            mp: 100,
            current_mp: 100,

        }
    }

    
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Class {
    pub name: String,
    pub class_model: ClassModel,
}

impl Class {
    pub fn new(name: &str, class_model: ClassModel) -> Self {
        Class {
            name: name.to_string(),
            class_model,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrimaryStats {
    pub strength: i32,
    pub intellect: i32,
    pub endurance: i32,
    pub dexterity: i32,
    pub wisdom: i32,
    pub luck: i32,
}
impl PrimaryStats {
    pub fn new(strength: i32, intellect: i32, endurance: i32, dexterity: i32, wisdom: i32, luck: i32) -> Self {
        PrimaryStats {
            strength,
            intellect,
            endurance,
            dexterity,
            wisdom,
            luck,
        }
    }

    pub fn add(&mut self, other: &PrimaryStats) {
        self.strength += other.strength;
        self.intellect += other.intellect;
        self.endurance += other.endurance;
        self.dexterity += other.dexterity;
        self.wisdom += other.wisdom;
        self.luck += other.luck;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecondaryStats {
    pub phy_out: f32,
    pub phy_in: f32,
    pub mag_out: f32,
    pub mag_in: f32,
    pub dot_out: f32,
    pub dot_in: f32,
    pub heal_out: f32,
    pub heal_in: f32,
    pub all_in: f32,
    pub all_out: f32,
    pub crit_chance: f32,
    pub crit_mod: f32,
    pub mana_consumption: f32,
    pub haste: f32,
    pub dodge: f32,
    pub hit_chance: f32,
    pub attack_power: f32,
    pub spell_power: f32,
    pub hp: i32,
    pub current_hp: i32,
    pub mp: i32,
    pub current_mp: i32,
}

impl SecondaryStats {
    pub fn new() -> Self {
        SecondaryStats {
            phy_out: 100.0,
            phy_in: 100.0,
            mag_out: 100.0,
            mag_in: 100.0,
            dot_out: 100.0,
            dot_in: 100.0,
            heal_out: 100.0,
            heal_in: 100.0,
            all_in: 100.0,
            all_out: 100.0,
            crit_chance: 5.0,
            crit_mod: 150.0,
            mana_consumption: 0.0,
            haste: 0.0,
            dodge: 4.0,
            hit_chance: 90.0,
            attack_power: 0.0,
            spell_power: 0.0,
            hp: 0,
            current_hp: 0,
            mp: 100,
            current_mp: 100

        }
    }

    pub fn add(&mut self, other: &SecondaryStats) {
        self.phy_out += other.phy_out;
        self.phy_in += other.phy_in;
        self.mag_out += other.mag_out;
        self.mag_in += other.mag_in;
        self.dot_out += other.dot_out;
        self.dot_in += other.dot_in;
        self.heal_out += other.heal_out;
        self.heal_in += other.heal_in;
        self.all_in += other.all_in;
        self.all_out += other.all_out;
        self.crit_chance += other.crit_chance;
        self.crit_mod += other.crit_mod;
        self.mana_consumption += other.mana_consumption;
        self.haste += other.haste;
        self.dodge += other.dodge;
        self.hit_chance += other.hit_chance;
        self.attack_power += other.attack_power;
        self.spell_power += other.spell_power;
        self.hp += other.hp;
        self.current_hp += other.current_hp;
        self.mp += other.mp;
        self.current_mp += other.current_mp;
    }

    pub fn modify_all_out(&mut self, value: f32, mult: bool) {
        if mult {
            self.all_out *= value;
        } else {
            self.all_out += value;
        }
    }

    pub fn modify_all_in(&mut self, value: f32, mult: bool) {
        if mult {
            self.all_in *= value;
        } else {
            self.all_in += value;
        }
    }

    pub fn modify_phy_out(&mut self, value: f32, mult: bool) {
        if mult {
            self.phy_out *= value;
        } else {
            self.phy_out += value;
        }
    }

    pub fn modify_phy_in(&mut self, value: f32, mult: bool) {
        if mult {
            self.phy_in *= value;
        } else {
            self.phy_in += value;
        }
    }

    pub fn modify_mag_out(&mut self, value: f32, mult: bool) {
        if mult {
            self.mag_out *= value;
        } else {
            self.mag_out += value;
        }
    }

    pub fn modify_mag_in(&mut self, value: f32, mult: bool) {
        if mult {
            self.mag_in *= value;
        } else {
            self.mag_in += value;
        }
    }

    pub fn modify_dot_out(&mut self, value: f32, mult: bool) {
        if mult {
            self.dot_out *= value;
        } else {
            self.dot_out += value;
        }
    }

    pub fn modify_dot_in(&mut self, value: f32, mult: bool) {
        if mult {
            self.dot_in *= value;
        } else {
            self.dot_in += value;
        }
    }

    pub fn modify_heal_out(&mut self, value: f32, mult: bool) {
        if mult {
            self.heal_out *= value;
        } else {
            self.heal_out += value;
        }
    }

    pub fn modify_heal_in(&mut self, value: f32, mult: bool) {
        if mult {
            self.heal_in *= value;
        } else {
            self.heal_in += value;
        }
    }

    pub fn modify_mana_consumption(&mut self, value: f32, mult: bool) {
        if mult {
            self.mana_consumption *= value;
        } else {
            self.mana_consumption += value;
        }
        if self.mana_consumption >= 50.0 {
            self.mana_consumption = 50.0;
        }
    }

    pub fn modify_hp(&mut self, value: i32) {
        self.hp = value;
        self.current_hp = value;
    }

    pub fn take_damage(&mut self, value: i32) {
        self.current_hp -= value;
    }

    pub fn use_mana(&mut self, value: i32) {
        self.current_mp -= value;
    }

}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerStats {
    pub primary: PrimaryStats,
    pub secondary: SecondaryStats,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    level: u32,
}

impl Player {
    pub fn new(level: u32) -> Self {
        Player {
            level,
        }
    }
    pub fn base_hp(&self) -> i32 {
        (((&self.level - 1)/(MAX_LEVEL - 1)) as f32).powf(0.66) as i32 * X_FACTOR as i32 + 360
    }

    pub fn efficiency(&self) -> f32 {
        1.0/((self.base_hp() as f32 * 63.0)/16000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_level_primary_stat_total() {
        let player = Player::new(100);
        let class_model = ClassModel::TankMelee;
        let primary_stats = class_model.level_primary_stat_total(&player);
        println!("Primary Stats at level 50 for Tank Melee: {:?}", primary_stats);

    }
}