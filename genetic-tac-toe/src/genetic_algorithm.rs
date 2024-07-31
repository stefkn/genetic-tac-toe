use rand::Rng;
use crate::tic_tac_toe::TicTacToe;
use crate::player::{Player, RandomComputerPlayer};

pub type Chromosome = Vec<usize>;

pub struct GeneticAlgorithm {
    pub population_size: usize, // The number of chromosomes in the population. Should be an even number.
    pub mutation_rate: f64, // The probability of a mutation occurring.
    pub crossover_rate: f64, // The probability of a crossover occurring.
    pub generations: usize, // The number of generations to evolve. 
    pub population: Vec<Chromosome>, // The population of chromosomes.
    // These should be such that population_size / 2^generations = 1. E.g. 128 / 2^7 = 1. 
    // because we will half the population size at each generation.
    // This ensures we are left with only one chromosome at the end of the evolution process. There can only be one!
}

impl GeneticAlgorithm {
    pub fn new(population_size: usize, mutation_rate: f64, crossover_rate: f64, generations: usize) -> Self {
        // Creates a new population of chromosomes-containing TicTacToe agents. 
        todo!()
    }

    pub fn fitness(&self, chromosome: &Chromosome) -> usize {
        // Returns the "fitness" of a chromosome, or how well it performs in the game.
        // The fitness is the number of games won by the chromosome.
        // The number of games played is equal to the population size.
        todo!()
    }

    pub fn selection(&self) -> Vec<Chromosome> {
        // Selects the best half of the chromosomes from the population.
        // The best chromosomes are selected based on their fitness.
        todo!()
    }

    pub fn crossover(&self, parent1: &Chromosome, parent2: &Chromosome) -> Chromosome {
        // Combines two chromosomes to create a new chromosome.
        todo!()
    }

    pub fn mutate(&self, chromosome: &mut Chromosome) {
        // Mutates a given chromosome in-place.
        todo!()
    }

    pub fn evolve(&mut self) {
        // Evolves the population of chromosomes.
        todo!()
    }
}

