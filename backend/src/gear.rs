use serde::{Serialize, Deserialize};


const GST_BASE: u8 = 12;
const STATS_EXPONENT: f32 = 1.33;
const GST_TOTAL: u16 = 572;
use crate::player::PrimaryStats;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Stat {
    Str,
    End,
    Dex,
    Int,
    Wis,
    Lck,
}

pub const STATS_ORDER: [Stat; 6] = [
    Stat::End,
    Stat::Str,
    Stat::Int,
    Stat::Dex,
    Stat::Wis,
    Stat::Lck,
];

impl Stat {
    pub fn name(&self) -> &'static str {
        match self {
            Stat::Str => "STR",
            Stat::End => "END",
            Stat::Dex => "DEX",
            Stat::Int => "INT",
            Stat::Wis => "WIS",
            Stat::Lck => "LCK",
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EnhancementPattern {
    Adventurer,
    Fighter,
    Thief,
    Armsman,
    Hybrid,
    Wizard,
    Healer,
    Spellbreaker,
    Lucky,
    Forge,
    Vim,
    Hearty,
    Examen,
    Pneuma,
    Anima,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Enhancement {
    pub level: u32,
    pub rarity: u32,
    pub pattern: EnhancementPattern,
}

impl Enhancement {
    pub fn new(level: u32, rarity: u32, pattern_id: &str) -> Option<Self> {
        EnhancementPattern::from_str(pattern_id).map(|pattern| Self {
            level,
            rarity,
            pattern,
        })
    }
}





#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PatternRatios {
    pub id: u32,
    pub name: &'static str,
    pub desc: &'static str,
    pub str_ratio: f32,
    pub dex_ratio: f32,
    pub end_ratio: f32,
    pub int_ratio: f32,
    pub wis_ratio: f32,
    pub lck_ratio: f32,
}

impl EnhancementPattern {
    pub fn ratios(&self) -> PatternRatios {
        use EnhancementPattern::*;
        match self {
            Adventurer => PatternRatios { id: 1, name: "Adventurer", desc: "none", str_ratio: 0.16, dex_ratio: 0.16, end_ratio: 0.18, int_ratio: 0.16, wis_ratio: 0.16, lck_ratio: 0.0 },
            Fighter    => PatternRatios { id: 2, name: "Fighter", desc: "M1", str_ratio: 0.44, dex_ratio: 0.13, end_ratio: 0.43, int_ratio: 0.0, wis_ratio: 0.0, lck_ratio: 0.0 },
            Thief      => PatternRatios { id: 3, name: "Thief", desc: "M2", str_ratio: 0.30, dex_ratio: 0.45, end_ratio: 0.25, int_ratio: 0.0, wis_ratio: 0.0, lck_ratio: 0.0 },
            Armsman    => PatternRatios { id: 4, name: "Armsman", desc: "M4", str_ratio: 0.38, dex_ratio: 0.36, end_ratio: 0.26, int_ratio: 0.0, wis_ratio: 0.0, lck_ratio: 0.0 },
            Hybrid     => PatternRatios { id: 5, name: "Hybrid", desc: "M3", str_ratio: 0.28, dex_ratio: 0.20, end_ratio: 0.25, int_ratio: 0.27, wis_ratio: 0.0, lck_ratio: 0.0 },
            Wizard     => PatternRatios { id: 6, name: "Wizard", desc: "C1", str_ratio: 0.0, dex_ratio: 0.0, end_ratio: 0.10, int_ratio: 0.50, wis_ratio: 0.20, lck_ratio: 0.20 },
            Healer     => PatternRatios { id: 7, name: "Healer", desc: "C2", str_ratio: 0.0, dex_ratio: 0.0, end_ratio: 0.40, int_ratio: 0.45, wis_ratio: 0.15, lck_ratio: 0.0 },
            Spellbreaker => PatternRatios { id: 8, name: "Spellbreaker", desc: "C3", str_ratio: 0.0, dex_ratio: 0.0, end_ratio: 0.20, int_ratio: 0.40, wis_ratio: 0.30, lck_ratio: 0.10 },
            Lucky      => PatternRatios { id: 9, name: "Lucky", desc: "S1", str_ratio: 0.10, dex_ratio: 0.10, end_ratio: 0.10, int_ratio: 0.10, wis_ratio: 0.10, lck_ratio: 0.50 },
            Forge      => PatternRatios { id: 10, name: "Forge", desc: "Blacksmith", str_ratio: 0.25, dex_ratio: 0.0, end_ratio: 0.0, int_ratio: 0.25, wis_ratio: 0.0, lck_ratio: 0.50 },
            Vim        => PatternRatios { id: 25, name: "Vim", desc: "SmithP2", str_ratio: 0.10, dex_ratio: 1.30, end_ratio: -0.90, int_ratio: 0.0, wis_ratio: 0.0, lck_ratio: 0.50 },
            Hearty     => PatternRatios { id: 32, name: "Hearty", desc: "Grimskull Troll Enhancement", str_ratio: -0.20, dex_ratio: -0.20, end_ratio: 1.50, int_ratio: -0.20, wis_ratio: -0.20, lck_ratio: -0.20 },
            Examen     => PatternRatios { id: 26, name: "Examen", desc: "SmithP2", str_ratio: 0.0, dex_ratio: 0.0, end_ratio: -0.90, int_ratio: 0.10, wis_ratio: 1.30, lck_ratio: 0.50 },
            Pneuma     => PatternRatios { id: 27, name: "Pneuma", desc: "SmithP2", str_ratio: 0.24, dex_ratio: 0.24, end_ratio: -0.90, int_ratio: 1.18, wis_ratio: 0.24, lck_ratio: 0.0 },
            Anima      => PatternRatios { id: 28, name: "Anima", desc: "SmithP2", str_ratio: 1.34, dex_ratio: 0.24, end_ratio: -0.90, int_ratio: 0.16, wis_ratio: 0.16, lck_ratio: 0.0 },
        }
    }

    pub fn from_id(id: u32) -> Option<Self> {
        use EnhancementPattern::*;
        Some(match id {
            1 => Adventurer,
            2 => Fighter,
            3 => Thief,
            4 => Armsman,
            5 => Hybrid,
            6 => Wizard,
            7 => Healer,
            8 => Spellbreaker,
            9 => Lucky,
            10 => Forge,
            25 => Vim,
            32 => Hearty,
            26 => Examen,
            27 => Pneuma,
            28 => Anima,
            _ => return None,
        })
    }

    pub fn from_str(s: &str) -> Option<Self> {
        use EnhancementPattern::*;
        Some(match s.to_ascii_lowercase().as_str() {
            "adventurer" => Adventurer,
            "fighter" => Fighter,
            "thief" => Thief,
            "armsman" => Armsman,
            "hybrid" => Hybrid,
            "wizard" => Wizard,
            "healer" => Healer,
            "spellbreaker" => Spellbreaker,
            "lucky" => Lucky,
            "forge" => Forge,
            "vim" => Vim,
            "hearty" => Hearty,
            "examen" => Examen,
            "pneuma" => Pneuma,
            "anima" => Anima,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum GearSlot {
    Helm,
    Armor,
    Cape,
    Weapon,
}

impl GearSlot {
    pub fn ratio(&self) -> f32 {
        match self {
            GearSlot::Helm => 0.25,   // "he"
            GearSlot::Armor => 0.25,  // "ar"
            GearSlot::Cape => 0.20,   // "ba"
            GearSlot::Weapon => 0.33, // "Weapon"
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "he" | "helm" => Some(GearSlot::Helm),
            "ar" | "armor" => Some(GearSlot::Armor),
            "ba" | "cape" | "back" => Some(GearSlot::Cape),
            "we" | "weapon" => Some(GearSlot::Weapon),
            _ => None,
        }
    }
}



#[derive(Debug)]
pub struct StatBlock {
    pub str_stat: i32,
    pub end_stat: i32,
    pub dex_stat: i32,
    pub int_stat: i32,
    pub wis_stat: i32,
    pub lck_stat: i32,
}

impl StatBlock {
    pub fn new() -> Self {
        Self {
            str_stat: 0,
            end_stat: 0,
            dex_stat: 0,
            int_stat: 0,
            wis_stat: 0,
            lck_stat: 0,
        }
    }

    pub fn sum(&self) -> i32 {
        self.str_stat + self.end_stat + self.dex_stat + self.int_stat + self.wis_stat + self.lck_stat
    }

    pub fn to_primary_stats(&self) -> PrimaryStats {
        PrimaryStats {
            strength: self.str_stat,
            endurance: self.end_stat,
            dexterity: self.dex_stat,
            intellect: self.int_stat,
            wisdom: self.wis_stat,
            luck: self.lck_stat,
        }
    }
}

pub fn gst_total(level: u32, rarity: u32) -> f32 {
    const GST_BASE: f32 = 12.0;
    const MAX_LEVEL: f32 = 100.0;

    let total = (level + rarity - 1) as f32;
    let value = GST_BASE + ((total * 560.0) / (MAX_LEVEL - 1.0));
    value.round().floor()
}


pub fn get_stats(enh: &Enhancement, slot: GearSlot) -> PrimaryStats {
    let gst_total_val = gst_total(enh.level, enh.rarity);
    let gear_stat_total = (gst_total_val * slot.ratio()).round() as i32;

    let ratios = enh.pattern.ratios();

    // Base stat distribution (floored percentages)
    println!("STR: {}\nGST: {}\nRATIO: {}", gear_stat_total as f32 * ratios.str_ratio, gear_stat_total as f32, ratios.str_ratio);
    let mut stats = StatBlock {
        str_stat: (gear_stat_total as f32 * ratios.str_ratio).round() as i32,
        dex_stat: (gear_stat_total as f32 * ratios.dex_ratio).round() as i32,
        end_stat: (gear_stat_total as f32 * ratios.end_ratio).round() as i32,
        int_stat: (gear_stat_total as f32 * ratios.int_ratio).round() as i32,
        wis_stat: (gear_stat_total as f32 * ratios.wis_ratio).round() as i32,
        lck_stat: (gear_stat_total as f32 * ratios.lck_ratio).round() as i32,
    };
    

    let mut other_count = stats.sum();
    println!("Other count: {}", other_count);

    // Fill remaining points to match the true GST total
    let mut index = 0;
    while other_count < gear_stat_total {
        match STATS_ORDER[index] {
            Stat::Str => stats.str_stat += 1,
            Stat::End => stats.end_stat += 1,
            Stat::Dex => stats.dex_stat += 1,
            Stat::Int => stats.int_stat += 1,
            Stat::Wis => stats.wis_stat += 1,
            Stat::Lck => stats.lck_stat += 1,
        }
        other_count += 1;

        index += 1;
        if index >= STATS_ORDER.len() {
            index = 0;
        }
    }

    stats.to_primary_stats()
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enhancement_pattern_from_id() {
        assert_eq!(EnhancementPattern::from_id(1).unwrap(), EnhancementPattern::Adventurer);
        assert_eq!(EnhancementPattern::from_id(6).unwrap(), EnhancementPattern::Wizard);
        assert_eq!(EnhancementPattern::from_id(28).unwrap(), EnhancementPattern::Anima);
        assert!(EnhancementPattern::from_id(99).is_none());
    }   
    #[test]
    fn test_gear_slot_from_str() {
        assert_eq!(GearSlot::from_str("he").unwrap(), GearSlot::Helm);
        assert_eq!(GearSlot::from_str("Armor").unwrap(), GearSlot::Armor);
        assert_eq!(GearSlot::from_str("BA").unwrap(), GearSlot::Cape);
        assert_eq!(GearSlot::from_str("weapon").unwrap(), GearSlot::Weapon);
        assert!(GearSlot::from_str("unknown").is_none());
    }

    #[test]
    fn test_get_stats() {
        let enh = Enhancement::new(100, 6, "anima").unwrap(); // Pattern 3 = Thief
        let slot = GearSlot::Helm;

        let stats = get_stats(&enh, slot);

        println!(
            "Helm ({:?}) enhancement {:?} → STR {}, DEX {}, END {}, INT {}, WIS {}, LCK {}",
            slot, enh.pattern,
            stats.strength, stats.dexterity, stats.endurance,
            stats.intellect, stats.wisdom, stats.luck
        );
    }
}