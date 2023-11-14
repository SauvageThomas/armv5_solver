use std::ops::AddAssign;

use log::{debug, trace};
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statistics {
    pub str: i8,
    pub sta: i8,
    pub dex: i8,
    pub qik: i8,

    pub int: i8,
    pub per: i8,
    pub pre: i8,
    pub com: i8,
}

pub const STR: Statistics = Statistics { str: 1, sta: 0, dex: 0, qik: 0, int: 0, per: 0, pre: 0, com: 0 };
pub const STA: Statistics = Statistics { str: 0, sta: 1, dex: 0, qik: 0, int: 0, per: 0, pre: 0, com: 0 };
pub const DEX: Statistics = Statistics { str: 0, sta: 0, dex: 1, qik: 0, int: 0, per: 0, pre: 0, com: 0 };
pub const QIK: Statistics = Statistics { str: 0, sta: 0, dex: 0, qik: 1, int: 0, per: 0, pre: 0, com: 0 };
pub const INT: Statistics = Statistics { str: 0, sta: 0, dex: 0, qik: 0, int: 1, per: 0, pre: 0, com: 0 };
pub const PER: Statistics = Statistics { str: 0, sta: 0, dex: 0, qik: 0, int: 0, per: 1, pre: 0, com: 0 };
pub const PRE: Statistics = Statistics { str: 0, sta: 0, dex: 0, qik: 0, int: 0, per: 0, pre: 1, com: 0 };
pub const COM: Statistics = Statistics { str: 0, sta: 0, dex: 0, qik: 0, int: 0, per: 0, pre: 0, com: 1 };

const NB_STATS: usize = 8;

impl Statistics {
    pub fn new() -> Self {
        Self { str: 0, sta: 0, dex: 0, qik: 0, int: 0, per: 0, pre: 0, com: 0 }
    }

    pub fn randomize(points: u8) -> Self {
        let mut stats = Self::new();
        trace!("randomize with {points} points");

        loop {
            stats.set_stats_at_random();
            if stats.calculate_cost().is_ok_and(|v| v <= points as i8) {
                break;
            }
        }
        debug!("Found real stats {:?} with", stats);

        stats
    }

    /// The mutate function will try to mutate some stats by decreasing some
    /// then increase some to fully ues all the points
    pub fn mutate(&self, mutation: u32, points: u8) -> Self {
        let mut statistics = self.clone();
        let rng = &mut thread_rng();
        let mut stats: [&mut i8; NB_STATS] = [
            &mut statistics.str, &mut statistics.sta, &mut statistics.dex, &mut statistics.qik,
            &mut statistics.int, &mut statistics.per, &mut statistics.pre, &mut statistics.com
        ];

        let mut current_cost = 0i8;
        // First, decrease some stats to gain some points
        for stats in &mut stats {
            if rng.gen_ratio(mutation, 100) {
                let decrease = Self::get_valid_decrease(stats, 0.3);
                **stats -= decrease;
            }
            current_cost += Self::get_cost(**stats).unwrap();
        }

        let mut nb_without_increase = 0u8;

        // Then, use all the remaining points to increase random stats
        loop {
            let remaining_points = points as i8 - current_cost;

            if remaining_points == 0 || nb_without_increase == 5 {
                return statistics;
            }


            let random_index = rng.gen_range(0..NB_STATS);
            let stat_to_mutate = stats.get_mut(random_index).unwrap();

            let increase = Self::get_valid_increase(stat_to_mutate, remaining_points, 3);

            if increase == 0 {
                nb_without_increase += 1;
            } else {
                nb_without_increase = 0;

                // Revert the cost of the stat
                current_cost -= Self::get_cost(**stat_to_mutate).unwrap();

                **stat_to_mutate += increase;

                // Apply the cost once the increase is made
                current_cost += Self::get_cost(**stat_to_mutate).unwrap();
            }
        }
    }

    fn get_valid_decrease(stat: &i8, random_ratio: f64) -> i8 {
        let mut rng = thread_rng();
        match Self::get_cost(stat - 1) {
            Ok(n) if rng.gen_bool(random_ratio) => match Self::get_cost(stat - 2) {
                Ok(n) if rng.gen_bool(random_ratio) => match Self::get_cost(stat - 3) {
                    Ok(n) if rng.gen_bool(random_ratio) => 3,
                    _ => 2,
                },
                _ => 1,
            },
            _ => 0,
        }
    }

    fn get_valid_increase(stat: &i8, remaining_points: i8, denominator: u32) -> i8 {
        let mut rng = thread_rng();
        let result = Self::get_cost(stat + 1);
        match Self::get_cost(stat + 1) {
            Ok(n) if n <= remaining_points => match Self::get_cost(stat + 2) {
                Ok(n) if rng.gen_ratio(1, denominator) && n <= remaining_points => match Self::get_cost(stat + 3) {
                    Ok(n) if rng.gen_ratio(1, denominator) && n <= remaining_points => 3,
                    _ => 2,
                },
                _ => 1,
            },
            _ => 0,
        }
    }

    fn mutate_stat(stat: &mut i8, mutation: u32, points: u8) {
        *stat = match thread_rng().gen_ratio(mutation, 100) {
            true => match thread_rng().gen_bool(0.5) {
                true => *stat - 1,
                false => *stat + 1,
            },
            false => *stat
        }
    }

    fn set_stats_at_random(&mut self) {
        let mut rng = thread_rng();
        self.str = rng.gen_range(-3..=3);
        self.sta = rng.gen_range(-3..=3);
        self.dex = rng.gen_range(-3..=3);
        self.qik = rng.gen_range(-3..=3);
        self.int = rng.gen_range(-3..=3);
        self.per = rng.gen_range(-3..=3);
        self.pre = rng.gen_range(-3..=3);
        self.com = rng.gen_range(-3..=3);
    }

    fn calculate_cost(&self) -> Result<i8, &str> {
        Ok(
            Self::get_cost(self.str)? +
                Self::get_cost(self.sta)? +
                Self::get_cost(self.dex)? +
                Self::get_cost(self.qik)? +
                Self::get_cost(self.int)? +
                Self::get_cost(self.per)? +
                Self::get_cost(self.pre)? +
                Self::get_cost(self.com)?
        )
    }

    fn get_cost(value: i8) -> Result<i8, &'static str> {
        match value {
            3 => Ok(6),
            2 => Ok(3),
            1 => Ok(1),
            0 => Ok(0),
            -1 => Ok(-1),
            -2 => Ok(-3),
            -3 => Ok(-6),
            _ => Err("A score cannot be that high at creation")
        }
    }

    pub fn retrieve_from_static(&self, cons: &Statistics) -> i8 {
        match *cons {
            STR => self.str,
            STA => self.sta,
            DEX => self.dex,
            QIK => self.qik,
            INT => self.int,
            PER => self.per,
            PRE => self.pre,
            COM => self.com,
            _ => panic!("What is that constant ??")
        }
    }
}

impl AddAssign for Statistics {
    fn add_assign(&mut self, rhs: Self) {
        self.str += rhs.str;
        self.sta += rhs.sta;
        self.dex += rhs.dex;
        self.qik += rhs.qik;

        self.int += rhs.int;
        self.per += rhs.per;
        self.pre += rhs.pre;
        self.com += rhs.com;
    }
}

pub fn add_ref(stats: &mut Statistics, o: &Statistics) {
    stats.str += o.str;
    stats.sta += o.sta;
    stats.dex += o.dex;
    stats.qik += o.qik;

    stats.int += o.int;
    stats.per += o.per;
    stats.pre += o.pre;
    stats.com += o.com;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_randomize() {
        let stats = Statistics::randomize(7);
    }

    #[test]
    fn test_max_increase() {
        let mut stat = Statistics::new();
        stat.dex = -2;
        // Make sure that the random part is always true
        let ratio = 1;
        assert_eq!(0, Statistics::get_valid_increase(&stat.str, 0, ratio));
        assert_eq!(1, Statistics::get_valid_increase(&stat.str, 1, ratio));
        assert_eq!(1, Statistics::get_valid_increase(&stat.str, 2, ratio));
        assert_eq!(2, Statistics::get_valid_increase(&stat.str, 3, ratio));
        assert_eq!(3, Statistics::get_valid_increase(&stat.str, 6, ratio));
        assert_eq!(3, Statistics::get_valid_increase(&stat.str, 100, ratio));

        assert_eq!(3, Statistics::get_valid_increase(&stat.dex, 100, ratio));
    }

    #[test]
    fn test() {
        let mut a = 0u8;

        let b = &mut a;

        *b += 1;

        println!("{a}");
    }

    #[test]
    fn test_mutate() {
        // init();
        let stats = Statistics { str: 0, sta: 0, dex: 0, qik: 0, int: -3, per: -3, pre: -3, com: -3 };
        println!("{:?}", stats);

        let i = Statistics::get_valid_increase(&-3, 8, 1);


        let mutated = stats.mutate(50, 7);
        println!("{:?}", mutated);
    }
}