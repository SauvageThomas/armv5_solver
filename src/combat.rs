use std::num::TryFromIntError;
use crate::character::Character;

use log::{debug, trace};
use crate::combat::Malus::{Ko, Safe, Wound};


#[derive(PartialEq)]
enum Malus {
    Safe,
    Wound(u8),
    Ko,
}

pub struct Combat;

impl Combat {
    pub fn run(c1: &Character, c2: &Character) -> bool {
        let init_c1 = c1.roll_init();
        let init_c2 = c2.roll_init();

        match init_c1 >= init_c2 {
            true => Self::do_1v1(c1, c2),
            false => !Self::do_1v1(c2, c1),
        }
    }

    fn do_1v1(c1: &Character, c2: &Character) -> bool {
        let mut i = 0;
        let mut malus_c1 = 0;
        let mut malus_c2 = 0;
        loop {
            debug!("Start of turn {i}");
            match Self::run_attack(c1, c2, malus_c1, malus_c2) {
                Safe => {}
                Wound(v) => malus_c2 += v,
                Ko => {
                    debug!("\n\n");
                    debug!("{} just won against {} !", c1.name, c2.name);
                    return true
                }
            }

            match Self::run_attack(c2, c1, malus_c2, malus_c1) {
                Safe => {}
                Wound(v) => malus_c1 += v,
                Ko => {
                    debug!("\n\n");
                    debug!("{} just won against {} !", c2.name, c1.name);
                    return false
                }
            }
            i += 1;
            debug!("\n\n");
        }
    }

    fn run_attack(c1: &Character, c2: &Character, malus_c1: u8, malus_c2: u8) -> Malus {
        let total_attack = c1.get_total_attack() - malus_c1 as i64;
        let damage = c1.weapon.damage as i64 + c1.stats.str as i64;

        if total_attack <= 0 || total_attack + damage <= 0 {
            debug!("There is no need to defend ... The attack is {total_attack} and damage is {damage}");
            return Safe
        }

        let total_defence = c2.get_total_defence() - malus_c2 as i64;

        if total_attack > total_defence {
            let soak = c2.default_soak + c2.armor.protection;
            let delta = (total_attack - total_defence) as u64;
            let damage_level = Self::get_damage_level(delta + damage as u64, soak, c2.size);
            if damage_level != 0 {
                let wound = match damage_level {
                    1 => "Light",
                    2 => "Medium",
                    3 => "Heavy",
                    4 => "Incapaciting",
                    _ => "Deadly"
                };
                debug!("{} takes a {wound} wound", c2.name);

                Self::get_malus_level(damage_level)
            } else {
                debug!("{} manages to fully soak the blow !", c2.name);
                Safe
            }
        } else {
            debug!("{} manages to fully parry the attack !", c2.name);
            Safe
        }
    }

    fn get_malus_level(level: u8) -> Malus {
        match level {
            0 => { Wound(0) }
            1 => { Wound(1) }
            2 => { Wound(3) }
            3 => { Wound(5) }
            _ => { Ko }
        }
    }

    fn get_damage_level(damage: u64, soak_calc: i8, size: i8) -> u8 {
        let wound_range: u64 = (5 + size) as u64;

        let soak = match soak_calc > 0 {
            true => soak_calc as u64,
            false => 0u64
        };

        trace!("Damage is {}, wound range is {}, soak = {}", damage, wound_range, soak);
        if damage <= soak {
            0
        } else {
            // The die can be very high bout there is no need for the wound to reflect it
            let result: Result<u8, TryFromIntError> = (((damage - soak - 1) / wound_range) + 1).try_into();
            match result {
                Ok(v) => v,
                Err(_) => u8::MAX,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use log::info;
    use crate::equipment::{POLE_ARM, POLE_AXE};
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_randomize() {
        init();

        let c1 = Character::randomize("toto".to_string());
        let c2 = Character::randomize("tata".to_string());

        let c1_winner = Combat::do_1v1(&c1, &c2);
        println!("c1 won {c1_winner}");
    }

}