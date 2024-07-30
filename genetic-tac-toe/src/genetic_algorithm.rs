use rand::Rng;
use crate::tic_tac_toe::TicTacToe;
use crate::player::{Player, RandomComputerPlayer};

pub type Chromosome = Vec<usize>;

pub struct GeneticAlgorithm {
    pub population_size: usize, // The number of chromosomes in the population.
    pub mutation_rate: f64, // The probability of a mutation occurring.
    pub crossover_rate: f64, // The probability of a crossover occurring.
    pub generations: usize, // The number of generations to evolve.
    pub population: Vec<Chromosome>, // The population of chromosomes.
}

impl GeneticAlgorithm {
    pub fn new(population_size: usize, mutation_rate: f64, crossover_rate: f64, generations: usize) -> Self {
        todo!()
    }

    pub fn fitness(&self, chromosome: &Chromosome) -> usize {
        todo!()
    }

    pub fn selection(&self) -> Vec<Chromosome> {
        todo!()
    }

    pub fn crossover(&self, parent1: &Chromosome, parent2: &Chromosome) -> Chromosome {
        todo!()
    }

    pub fn mutate(&self, chromosome: &mut Chromosome) {
        todo!()
    }

    pub fn evolve(&mut self) {
        todo!()
    }
}

