mod character;
mod equipment;
mod dies;
mod brain;
mod stats;
mod island;
mod combat;

use std::{
    thread,
    time,
    time::Instant,
};
use log::info;
use crate::stats::{STA, Statistics, STR};
use crate::character::{Character};
use crate::character::Virtue::{GiantBlood, GreatCharacteristics, PuissantAbility, Large, Tough, EnduringConstitution, AffinityAbility};
use crate::combat::Combat;
use crate::equipment::{FULL_CHAIN_MAIL, GREAT_SWORD, PLATE_FULL_MAIL_CAMAIL_LVL_1, PLATE_FULL_MAIL_LVL_1, POLE_ARM, POLE_AXE};
use crate::island::Island;


fn main() {
    env_logger::init();
    let now = Instant::now();

    let nb_island = 1;
    let nb_run = 100u32;
    let lambda = 1000;
    let mutation = 0.1f32;

    for i in 0..nb_island {
        println!("Creating island n°{i}");
        let island = Island::new(format!("{i}"), lambda, mutation);

        println!("Running island n°{i}");
        island.run(nb_run);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}


fn custom_1v1() {
    let mut toto = Character::new(
        "Indigo Pyrithe".to_string(), 1,
        &[&GiantBlood, &GreatCharacteristics(STA), &PuissantAbility, &AffinityAbility],
        Statistics { str: 2, sta: 2, dex: 3, qik: 4, int: 0, per: 0, pre: 0, com: 0 },
        5,
        &GREAT_SWORD, &PLATE_FULL_MAIL_CAMAIL_LVL_1,
    );

    // let mut target = Character::new(
    //     "Foe", 1,
    //     &[Tough, Tall, GreatCharacteristics(Statistics::sta(2)), GreatCharacteristics(Statistics::str(1))],
    //     Statistics { str: 4, sta: 2, dex: 2, qik: 3 },
    //     7, true,
    //     &POLE_AXE, &FULL_CHAIN_MAIL,
    // );

    let mut target = Character::new(
        "Foe".to_string(), 1,
        &[&Tough, &GiantBlood, &GreatCharacteristics(STA), &PuissantAbility, &AffinityAbility],
        Statistics { str: 2, sta: 2, dex: 3, qik: 4, int: 0, per: 0, pre: 0, com: 0 },
        6,
        &GREAT_SWORD, &PLATE_FULL_MAIL_CAMAIL_LVL_1,
    );

    let mut manon = Character::new(
        "Sparkle Minouchatte".to_string(), 2,
        &[&Tough, &GiantBlood, &GreatCharacteristics(STA), &EnduringConstitution],
        Statistics { str: 5, sta: 5, dex: 5, qik: 5, int: 5, per: 5, pre: 5, com: 5 },
        7,
        &GREAT_SWORD, &PLATE_FULL_MAIL_CAMAIL_LVL_1,
    );

    println!("####################");
    println!("Combat starts");
    println!("{:?}", toto);
    println!("vs");
    println!("{:?}", manon);

    let mut c1_winner = 0;
    let mut c2_winner = 0;
    for i in 0..1 {
        let c1_wins = Combat::run(&toto, &manon);
        if c1_wins {
            c1_winner += 1;
        } else {
            c2_winner += 1;
        }
    }

    println!("{} won {c1_winner} times and {} won {c2_winner} times", toto.name, target.name);
    let ratio = (c1_winner as f32 / (c1_winner + c2_winner) as f32) * 100f32;
    println!("{} won with {ratio}% ratio", toto.name);
}