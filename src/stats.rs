use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::ops::AddAssign;
use log::{debug, info, trace};

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

pub const STR: Statistics = Statistics{ str: 1, sta: 0, dex: 0, qik: 0, int: 0, per: 0, pre: 0, com: 0 };
pub const STA: Statistics = Statistics{ str: 0, sta: 1, dex: 0, qik: 0, int: 0, per: 0, pre: 0, com: 0 };
pub const DEX: Statistics = Statistics{ str: 0, sta: 0, dex: 1, qik: 0, int: 0, per: 0, pre: 0, com: 0 };
pub const QIK: Statistics = Statistics{ str: 0, sta: 0, dex: 0, qik: 1, int: 0, per: 0, pre: 0, com: 0 };
pub const INT: Statistics = Statistics{ str: 0, sta: 0, dex: 0, qik: 0, int: 1, per: 0, pre: 0, com: 0 };
pub const PER: Statistics = Statistics{ str: 0, sta: 0, dex: 0, qik: 0, int: 0, per: 1, pre: 0, com: 0 };
pub const PRE: Statistics = Statistics{ str: 0, sta: 0, dex: 0, qik: 0, int: 0, per: 0, pre: 1, com: 0 };
pub const COM: Statistics = Statistics{ str: 0, sta: 0, dex: 0, qik: 0, int: 0, per: 0, pre: 0, com: 1 };

impl Statistics {
    pub fn new() -> Self {
        Self { str: 0, sta: 0, dex: 0, qik: 0, int: 0, per: 0, pre: 0, com: 0 }
    }

    pub fn randomize(points: u8) -> Self {
        let mut stats = Self::new();
        trace!("randomize with {points} points");

        loop {
            stats.set_random_stat();
            let total_cost = stats.get_total_cost();
            trace!("{:?} with {total_cost} points", stats);
            if total_cost > 0 && total_cost <= points as i8 {
                break;
            }
        }
        debug!("Found real stats {:?} with {} points", stats, stats.get_total_cost());

        stats
    }

    fn set_random_stat<'a>(&mut self) {
        self.str = thread_rng().gen_range(-3..=3);
        self.sta = thread_rng().gen_range(-3..=3);
        self.dex = thread_rng().gen_range(-3..=3);
        self.qik = thread_rng().gen_range(-3..=3);
        self.int = thread_rng().gen_range(-3..=3);
        self.per = thread_rng().gen_range(-3..=3);
        self.pre = thread_rng().gen_range(-3..=3);
        self.com = thread_rng().gen_range(-3..=3);
    }

    fn get_total_cost(&self) -> i8 {
        Self::get_cost(self.str) +
            Self::get_cost(self.sta) +
            Self::get_cost(self.dex) +
            Self::get_cost(self.qik) +
            Self::get_cost(self.int) +
            Self::get_cost(self.per) +
            Self::get_cost(self.pre) +
            Self::get_cost(self.com)
    }

    fn get_cost(value: i8) -> i8 {
        match value {
            3 => 6,
            2 => 3,
            1 => 1,
            0 => 0,
            -3 => -1,
            -2 => -3,
            -1 => -6,
            _ => panic!("A score cannot be that high at creation")
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


    #[test]
    fn test_randomize() {
        let stats = Statistics::randomize(7);
    }
}