use crate::dies::roll_stress_die;
use crate::dies::StressDie::{BOTCH, DIE};
use crate::equipment::{Armor, CUSTOM, GREAT_SWORD, PLATE_FULL_MAIL_CAMAIL_LVL_1, Weapon};

use log::{debug, log, trace};
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

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub size: i8,
    pub default_size: i8,
    pub default_soak: i8,
    // Stats
    pub stats: Statistics,
    pub base_stats: Statistics,
    pub martial_ability: u8,
    pub is_ability_puissant: bool,
    pub is_enduring: bool,
    pub weapon: &'static Weapon,
    pub armor: &'static Armor,
    pub virtues: Vec<&'static Virtue>,
}

const DEFAULT_MARTIAL_ABILITY: u8 = 6;

impl Character {
    pub fn new(name: String, race_size: i8, virtues: Vec<&'static Virtue>, stats: Statistics,
               mut martial_ability: u8, weapon: &'static Weapon, armor: &'static Armor) -> Self {
        let default_size = race_size;
        let base_stats = stats.clone();

        let (size, default_soak) = (race_size, 0i8);

        let is_ability_puissant = false;
        let is_enduring = false;

        let mut character = Self {
            name,
            size,
            default_size,
            default_soak,
            stats,
            base_stats,
            martial_ability,
            is_ability_puissant,
            is_enduring,
            weapon,
            armor,
            virtues,
        };


        debug!("Before apply: {:?}", character);
        character.apply_virtues();
        debug!("After apply: {:?}", character);

        character
    }

    fn apply_virtues(&mut self) {
        self.size = self.default_size;
        self.default_soak = 0;
        self.stats = self.base_stats.clone();
        self.is_ability_puissant = false;
        self.is_enduring = false;

        for virtue in &self.virtues {
            match virtue {
                Tough => { self.default_soak += 3 }
                AffinityAbility => { self.martial_ability += 1 }
                PuissantAbility => { self.is_ability_puissant = true }
                EnduringConstitution => { self.is_enduring = true }
                GiantBlood => {
                    self.size += 2;
                    self.stats += STR;
                    self.stats += STA;
                }
                Large => { self.size += 1 }
                ImprovedCharacteristics => {} // Nothing to do, it has been done to stats before
                GreatCharacteristics(s) => { add_ref(&mut self.stats, &s) }
                None => {}
            }
        }

        self.default_soak += self.stats.sta;
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
            DEFAULT_MARTIAL_ABILITY,
            &GREAT_SWORD, &CUSTOM,
        );

        character.virtues = virtues;
        character.randomize_virtues(character.virtues.len() as u8);
        character.apply_virtues();

        character
    }

    pub fn mutate(&self, mutation: u32, i: usize) -> Character {
        let mut mutated = (*self).clone();
        mutated.martial_ability = DEFAULT_MARTIAL_ABILITY;
        mutated.name = i.to_string();
        debug!("Previous virtues {:?}, with size", mutated.virtues);
        // Check if the virtue is mutating
        mutated.virtues.retain(|&v| !thread_rng().gen_ratio(mutation, 100));
        debug!("Retained virtues {:?}", mutated.virtues);

        let mut total_virtues = mutated.count_virtues();

        let remaining_virtues = POINTS_VIRTUES - total_virtues;
        // The array + ImprovedCharacteristics
        let nb_virtues = POSSIBLE_VIRTUES.len() + 1;

        // This virtue must be done before great characs and stats mutation
        for _ in 0..remaining_virtues {
            if thread_rng().gen_range(0..nb_virtues) == 0 {
                mutated.virtues.push(&ImprovedCharacteristics);
                total_virtues += 1;
            }
        }
        debug!("Retained virtues with ImprovedCharacteristics {:?}", mutated.virtues);

        debug!("Base Stats before {:?}", mutated.base_stats);
        mutated.base_stats.mutate(mutation, POINTS_VIRTUES);
        debug!("Base Stats after {:?}", mutated.base_stats);
        mutated.stats = mutated.base_stats.clone();

        mutated.randomize_virtues(total_virtues);
        debug!("New virtues {:?}", mutated.virtues);

        mutated.apply_virtues();

        mutated
    }

    fn randomize_virtues(&mut self, mut total_virtues: u8) {
        let mut i = 0u8;
        loop {
            if i > 5 || total_virtues == POINTS_VIRTUES {
                break;
            }

            let pick = &POSSIBLE_VIRTUES[thread_rng().gen_range(0..POSSIBLE_VIRTUES.len())];
            debug!("Picking {:?}", pick);

            if self.is_valid(pick, &self.virtues, &total_virtues) {
                i = 0;
                debug!("Valid");
                match pick {
                    GiantBlood => total_virtues += 3,
                    _ => total_virtues += 1,
                }
                // character.apply_virtue(pick);
                self.virtues.push(pick);
            } else {
                i += 1;
                debug!("Not valid");
            }

            debug!("total_virtues {total_virtues}, {:?}", self.virtues);
        }
    }

    fn count_virtues(&self) -> u8 {
        let count = self.virtues
            .iter()
            .map(|&v| match v {
                GiantBlood => 3,
                _ => 1
            })
            .reduce(|acc, v| acc + v);

        match count {
            Some(i) => i,
            _ => 0
        }
    }

    fn is_valid(&self, virtue: &Virtue, virtues: &[&Virtue], total_virtues: &u8) -> bool {
        match virtue {
            Large => !virtues.iter().any(|v| **v == GiantBlood || **v == Large),
            GiantBlood => *total_virtues <= POINTS_VIRTUES - 3 && !virtues.iter().any(|v| **v == GiantBlood || **v == Large),
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

    #[test]
    fn test_mutate() {
        init();

        let mut char = Character::randomize("toto".to_string());
        info!("Init: {:?}", char);
        info!("\n\nMutation ... \n\n");

        char.mutate(35, 0);

        info!("Mutated: {:?}", char);
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



