use crate::dies::roll_stress_die;
use crate::dies::StressDie::{BOTCH, DIE};
use crate::equipment::{Armor, CUSTOM, GREAT_SWORD, PLATE_FULL_MAIL_CAMAIL_LVL_1, Weapon};

use log::{debug, trace};
use log::error;
use log::info;
use log::warn;
use rand::thread_rng;
use rand::Rng;
use crate::character::Virtue::*;
use crate::stats::{add_ref, COM, DEX, INT, PER, PRE, QIK, STA, Statistics, STR};

const POSSIBLE_VIRTUES: [Virtue; 14] = [
    Tough,
    GiantBlood,
    Large,
    GreatCharacteristics(STR), GreatCharacteristics(STA), GreatCharacteristics(QIK), GreatCharacteristics(DEX),
    GreatCharacteristics(PER), GreatCharacteristics(PRE), GreatCharacteristics(INT), GreatCharacteristics(COM),
    PuissantAbility,
    AffinityAbility,
    EnduringConstitution,
];


const POINTS_VIRTUES: u8 = 8u8;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub size: i8,
    pub default_soak: i8,
    // Stats
    pub stats: Statistics,
    pub martial_ability: u8,
    pub is_ability_puissant: bool,
    pub is_enduring: bool,
    pub weapon: &'static Weapon,
    pub armor: &'static Armor,
    pub virtues: Vec<&'static Virtue>,
}

impl Character {
    pub fn new(name: String, race_size: i8, virtues: Vec<&'static Virtue>, mut stats: Statistics,
               mut martial_ability: u8, weapon: &'static Weapon, armor: &'static Armor) -> Self {
        let (mut size, mut default_soak) = (race_size, 0i8);

        let mut is_ability_puissant = false;
        let mut is_enduring = false;


        for virtue in &virtues {
            match virtue {
                Tough => { default_soak += 3 }
                AffinityAbility => { martial_ability += 1 }
                PuissantAbility => { is_ability_puissant = true }
                EnduringConstitution => { is_enduring = true }
                GiantBlood => {
                    size += 2;
                    stats += STR;
                    stats += STA;
                }
                Large => { size += 1 }
                ImprovedCharacteristics => {} // Nothing to do, it has been done to stats before
                GreatCharacteristics(s) => { add_ref(&mut stats, &s) }
                None => {}
            }
        }

        default_soak += stats.sta;

        let mut character = Self {
            name,
            size,
            default_soak,
            stats,
            martial_ability,
            is_ability_puissant,
            is_enduring,
            weapon,
            armor,
            virtues,
        };

        character
    }


    pub fn randomize(name: String) -> Self {
        // The array + ImprovedCharacteristics
        let nb_virtues = POSSIBLE_VIRTUES.len() + 1;
        let mut virtues = Vec::with_capacity(POINTS_VIRTUES as usize);
        // This virtue must be done before great characs and stats randomization
        for _ in 0..POINTS_VIRTUES {
            if thread_rng().gen_range(0..nb_virtues) == 0 {
                virtues.push(&ImprovedCharacteristics);
            }
        }

        debug!("First virtues: {:?}", virtues);

        let stats = Statistics::randomize(7u8 + 3 * virtues.len() as u8);

        let mut character = Self::new(
            name,
            0,
            vec![],
            stats,
            6,
            &GREAT_SWORD, &CUSTOM,
        );

        let mut total_virtues = virtues.len() as u8;
        let mut i = 0u8;
        loop {
            let pick = &POSSIBLE_VIRTUES[thread_rng().gen_range(0..POSSIBLE_VIRTUES.len())];
            debug!("Picking {:?}", pick);

            if character.is_valid(pick, &virtues, &total_virtues) {
                i = 0;
                debug!("Valid");
                match pick {
                    GiantBlood => total_virtues += 3,
                    _ => total_virtues += 1,
                }
                // character.apply_virtue(pick);
                virtues.push(pick);
            } else {
                i += 1;
                debug!("Not valid");
            }

            debug!("total_virtues {total_virtues}, {:?}", virtues);

            if i > 5 || total_virtues == POINTS_VIRTUES {
                break;
            }
        }

        character.virtues = virtues;

        character
    }

    pub fn mutate(&mut self, mutation: u32) {
        self.stats.mutate(mutation, POINTS_VIRTUES);

        todo!(Faire en sorte de muter le personnage tout en conservant les valeurs des stats AVANT les vertues)
    }

    fn is_valid(&self, virtue: &Virtue, virtues: &[&Virtue], total_virtues: &u8) -> bool {
        match virtue {
            Large => !virtues.iter().any(|v| **v == GiantBlood || **v == Large),
            GiantBlood => *total_virtues <= 7 && !virtues.iter().any(|v| **v == GiantBlood || **v == Large),
            Tough => !virtues.contains(&&Tough),
            ImprovedCharacteristics => true,
            GreatCharacteristics(s) => {
                let value = self.stats.retrieve_from_static(s);
                value == 3 || value == 4
            }
            PuissantAbility => !virtues.contains(&&PuissantAbility),
            AffinityAbility => !virtues.contains(&&AffinityAbility),
            EnduringConstitution => !virtues.contains(&&EnduringConstitution),
            None => panic!("Should not happen")
        }
    }

    fn get_total(&self, stress_die: u64, stat: i64, weapon: i64, exert: bool) -> i64 {
        let mut ability = self.martial_ability;
        if exert {
            ability += self.martial_ability;
        }
        if self.is_ability_puissant {
            ability += 2
        }

        debug!("ability is {ability}, die is {stress_die}, char is {stat}, weapon is {weapon}, exert is {exert}");

        stress_die as i64 + stat + weapon + ability as i64
    }

    pub fn roll_init(&self) -> i64 {
        debug!("{} rolls init", self.name);
        match roll_stress_die(1) {
            BOTCH => {
                debug!("{} rolls a botch !", self.name);
                0
            }
            DIE(stress_die) => {
                let init = self.get_total(stress_die, self.stats.qik.into(), self.weapon.init.into(), false);
                debug!("{} rolls a {stress_die} ! Total init is {init}", self.name);
                init
            }
        }
    }

    pub fn get_total_attack(&self) -> i64 {
        debug!("{} attacks", self.name);
        match roll_stress_die(1) {
            BOTCH => {
                debug!("{} rolls a botch !", self.name);
                0
            }
            DIE(stress_die) => {
                let exert = false;
                let attack = self.get_total(stress_die, self.stats.dex.into(), self.weapon.attack.into(), exert);
                debug!("{} rolls a {stress_die} ! Total attack is {attack}", self.name);
                attack
            }
        }
    }

    pub fn get_total_defence(&self) -> i64 {
        debug!("{} defends against a blow", self.name);
        match roll_stress_die(1) {
            BOTCH => {
                debug!("{} rolls a botch !", self.name);
                0
            }
            DIE(stress_die) => {
                let exert = false;
                let defence = self.get_total(stress_die, self.stats.qik.into(), self.weapon.defence.into(), exert);
                debug!("{} rolls a {stress_die} ! Total defence is {defence}", self.name);
                defence
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Virtue {
    Tough,
    GiantBlood,
    Large,
    ImprovedCharacteristics,
    GreatCharacteristics(Statistics),
    PuissantAbility,
    AffinityAbility,
    EnduringConstitution,
    None,
}

#[cfg(test)]
mod tests {
    use crate::equipment::{POLE_ARM, POLE_AXE};
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_randomize() {
        init();

        let char = Character::randomize("toto".to_string());
        info!("{:?}", char);
    }

    // #[test]
    // fn test_clone() {
    //     init();
    //
    //     let mut c1 = Character::randomize("toto".to_string());
    //     let mut c2 = c1.clone();
    //     println!("{:?}", c1);
    //     println!("{:?}", c2);
    //
    //     println!("{:p}", &c1);
    //     println!("{:p}", &c2);
    // }
}



