use std::str::FromStr;

use crate::{error::BackendError, player::{PrimaryStats, SecondaryStats}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Weapon {
    pub range: f32,
    pub dps: f32,
    pub boost: WeaponBoost
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[allow(nonstandard_style)]
pub enum DamageSource {
    AP1,
    SP1,
    AP2,
    SP2,
    APSP1,
    APSP2,
    cHPm,
    cMPm,
    intHP,
    intMP,
}


impl DamageSource {
    fn compute(&self, weapon: &Weapon, stat: &SecondaryStats) -> f32 {
        match self {
            DamageSource::AP1 => weapon.dps + (0.1 * stat.attack_power),
            DamageSource::AP2 => 2.0 * DamageSource::AP1.compute(weapon, stat) * weapon.range,
            DamageSource::SP1 => weapon.dps + (0.1 * stat.spell_power),
            DamageSource::SP2 => 2.0 * DamageSource::SP1.compute(weapon, stat) * weapon.range,
            DamageSource::APSP1 => weapon.dps + (0.1 * DamageSource::AP1.compute(weapon, stat) ) + (0.1 * DamageSource::SP1.compute(weapon, stat)),
            DamageSource::APSP2 => 2.0 * DamageSource::APSP1.compute(weapon, stat),
            DamageSource::cHPm => stat.hp as f32,
            DamageSource::cMPm => stat.mp as f32,
            DamageSource::intHP => stat.current_hp as f32,
            DamageSource::intMP => stat.current_mp as f32,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeaponBoost {
    Boost15,
    Boost30,
    Boost51,
    Boost51x30,
    Boost51x40,
    Boost51x50,
    Boost35x75,
    Custom(f32)
}

impl FromStr for WeaponBoost {
    type Err = BackendError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            "15" => Ok(WeaponBoost::Boost15),
            "30" => Ok(WeaponBoost::Boost30),
            "51" => Ok(WeaponBoost::Boost51),
            "51x30" => Ok(WeaponBoost::Boost51x30),
            "51x40" => Ok(WeaponBoost::Boost51x40),
            "51x50" => Ok(WeaponBoost::Boost51x50),
            "35x75" => Ok(WeaponBoost::Boost35x75),
            _ => {
                match s.parse::<f32>() {
                    Ok(v) => Ok(WeaponBoost::Custom(1.0 + (v / 100.0))),
                    Err(e) => Err(BackendError::InvalidWeaponBoost(format!("Error: {e} ")))
                }
                

            } 
        }
    }
}
impl WeaponBoost {
    pub fn multiplier(&self) -> f32 {
        match self {
            WeaponBoost::Boost15 => 1.15,
            WeaponBoost::Boost30 => 1.30,
            WeaponBoost::Boost51 => 1.51,
            WeaponBoost::Boost51x30 => 1.51 * 1.3,
            WeaponBoost::Boost51x40 => 1.51 * 1.4,
            WeaponBoost::Boost51x50 => 1.51 * 1.5,
            WeaponBoost::Boost35x75 => 1.35 * 1.75,
            WeaponBoost::Custom(x) => *x,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Type {
    Physical,
    Magical,
    TrueDamage,
    DamageOverTime,
}

impl Type {
    pub fn self_modifiers(&self, stat: &SecondaryStats, crit: bool) -> f32 {
        let crit_mod = if crit { stat.crit_mod } else { 100.0 };

        match self {
            Type::Physical => (stat.all_out / 100.0) * (stat.phy_out / 100.0) * (crit_mod / 100.0),
            Type::Magical => (stat.all_out / 100.0) * (stat.mag_out / 100.0) * (crit_mod / 100.0),
            Type::TrueDamage => 1.0 * (crit_mod / 100.0),
            // This isn't proper I'm just lazy
            Type::DamageOverTime => (stat.all_out / 100.0).powi(2) * (stat.mag_out / 100.0) * (stat.dot_out / 100.0)
        }
    }

}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Target {
    Yourself,
    Enemy,
    Friendly
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ForceResult {
    Hit,
    Crit,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Properties {
    force_result: Option<ForceResult>,
    add_crit: Option<f32>,
    mana_back: Option<u32>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Skill {
    dsrc: DamageSource,
    damage_type: Type,
    cd: u32,
    mp: u32,
    target: Target,
    properties: Properties
}