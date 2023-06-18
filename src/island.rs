use std::fmt::format;
use std::time::Instant;
use crate::character::Character;
use log::{debug, info};
use crate::combat::Combat;

pub struct Island {
    pub name: String,
    pub mutation: f32,
    lambda: usize,
    population: Vec<Character>,
}

impl <'a> Island{
    pub fn new(name: String, lambda: usize, mutation: f32) -> Self {
        let mut population = Vec::with_capacity(lambda);
        for i in 0..lambda {
            population.push(Character::randomize(format!("{i}")));
            // population.push(Character::randomize(""));
        }
        info!("Randomize done");

        Island {
            name,
            mutation,
            lambda,
            population,
        }
    }

    pub fn run(&self, nb_run: u32) -> &Character {

        // Character::randomize("")
        Self::do_1_generation(&self.population)
        // for i in 0..nb_run {
        //     self.do_1_generation()
        //     return self.do_1_generation()
        //     // return best_char = self.do_1_generation();
        // }
    }

    fn do_1_generation(population: &'a Vec<Character>) -> &'a Character {
        info!("Calculate fitness for generation ...");
        // let cloned_pop = population.clone();

        match population
            .iter()
            .map(|c| Self::calculate_fitness(population, c))
            .reduce(|(c1, fit1), (c2, fit2)|
                match fit1 > fit2 {
                    true => (c1, fit1),
                    false => (c2, fit2),
                }
            ) {
            Some((char, best_fitness)) => {
                println!("Best character is {:?} with fitness {best_fitness}", char);
                char
            }
            None => panic!("Result cannot be empty")
        }
    }

    fn calculate_fitness(population: &'a Vec<Character>, char: &'a Character) -> (&'a Character, f32) {
        // let name = char.name.as_str();

        let now = Instant::now();
        let fitness = population
            .iter()
            .filter(|c| c.name != char.name)
            .map(|c| Self::run_fights(char, c))
            .sum::<f32>() / population.len() as f32;

        let elapsed = now.elapsed();
        // println!("Elapsed: {:.2?} for {}, fitness: {fitness}", elapsed, char.name);
        (char, fitness)
    }

    fn run_fights(c1: &Character, c2: &Character) -> f32 {
        // info!("Calculate fitness for {} vs {}...", c1.name, c2.name);
        let nb_fight = 1000u32;

        (0..nb_fight)
            .map(|_| Combat::run(c1, c2))
            .filter(|v| *v)
            .count() as f32 / nb_fight as f32
    }
}


