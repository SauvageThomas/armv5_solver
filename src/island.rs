use std::fmt::format;
use std::time::Instant;
use crate::character::Character;
use log::{debug, info};
use crate::combat::Combat;

pub struct Island {
    pub name: String,
    pub mutation: u32,
    lambda: usize,
    population: Vec<Character>,
}

impl<'a> Island {
    pub fn new(name: String, lambda: usize, mutation: u32) -> Self {
        let mut population = Vec::with_capacity(lambda);
        for i in 0..lambda {
            population.push(Character::randomize(format!("{i}")));
        }
        info!("Randomize done");

        Island {
            name,
            mutation,
            lambda,
            population,
        }
    }

    pub fn run(&mut self, nb_run: u32) {
        for i in 0..nb_run + 1 {
            info!("Running generation {i}");
            let best = Self::do_1_generation(&self.population);

            // Mutate the population based on the best
            self.population.clear();
            for i in 0..self.lambda {
                self.population.push(best.mutate(self.mutation, i));
            }
        }
    }

    fn do_1_generation(population: &'a Vec<Character>) -> Character {
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
                char.clone()
            }
            None => panic!("Result cannot be empty")
        }
    }

    fn calculate_fitness(population: &'a Vec<Character>, char: &'a Character) -> (&'a Character, f32) {
        let fitness = population
            .iter()
            .filter(|c| c.name != char.name)
            .map(|c| Self::run_fights(char, c))
            .sum::<f32>() / population.len() as f32;

        (char, fitness)
    }

    fn run_fights(c1: &Character, c2: &Character) -> f32 {
        let nb_fight = 100u32;

        (0..nb_fight)
            .map(|_| Combat::run(c1, c2))
            .filter(|v| *v)
            .count() as f32 / nb_fight as f32
    }
}


