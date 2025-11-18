use serde::{Serialize, Deserialize};


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
            phy_out: 0.0,
            phy_in: 0.0,
            mag_out: 0.0,
            mag_in: 0.0,
            dot_out: 0.0,
            dot_in: 0.0,
            heal_out: 0.0,
            heal_in: 0.0,
            all_in: 0.0,
            all_out: 0.0,
            crit_chance: 15.0,
            crit_mod: 200.0,
            mana_consumption: 0.0,
            haste: 37.5,
            dodge: 10.0,
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