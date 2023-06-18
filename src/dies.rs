use rand::thread_rng;
use rand::Rng;
use crate::dies::StressDie::{BOTCH, DIE};

pub fn roll_die() -> u8 {
    thread_rng().gen_range(0..10)
}

pub enum StressDie {
    BOTCH,
    DIE(u64),
}

pub fn roll_stress_die(botch_die: u8) -> StressDie {
    match roll_die() {
        0 => { check_botch(botch_die) }
        1 => { roll_crit(1) }
        i @ 2..=9 => DIE(i.into()),
        _ => { panic!("Should not be happening") }
    }
}

pub fn roll_crit(factor: u32) -> StressDie {
    match roll_die() {
        0 => { DIE(10 * 2u64.pow(factor)) }
        1 => { roll_crit(factor + 1) }
        i @ 2..=9 => DIE(i as u64 * 2u64.pow(factor)),
        _ => { panic!("Should not be happening") }
    }
}

fn check_botch(botch_die: u8) -> StressDie {
    if (1..=botch_die).map(|_| roll_die()).any(|v| v == 0) {
        BOTCH
    } else {
        DIE(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_roll() {
        let outside_bounds = (1..=100).map(|_| roll_die()).any(|v| v > 9);
        assert_eq!(false, outside_bounds);
    }

    #[test]
    fn test_stress_die() {
        for _ in 0..10000 {
            println!("###################");
            match roll_stress_die(3) {
                // BOTCH => println!("Rolled a botch !"),
                BOTCH => {}
                DIE(i) => {
                    if i > 20 {
                        println!("..............................................");
                        println!("Result of stress die {i}")
                    }
                }
            }
        }
    }
}