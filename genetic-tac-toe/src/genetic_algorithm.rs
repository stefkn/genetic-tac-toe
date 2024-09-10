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
        let population = (0..population_size)
            .map(|_| (0..9).map(|_| rand::thread_rng().gen_range(0..9)).collect())
            .collect();

        Self {
            population_size,
            mutation_rate,
            crossover_rate,
            generations,
            population,
        }
    }

    pub fn fitness(&self, chromosome: &Chromosome) -> usize {
        // Returns the "fitness" of a chromosome, or how well it performs in the game.
        // The fitness is the number of games won by the chromosome.
        // The number of games played is equal to the population size.
        let mut wins = 0;
        for _ in 0..10 {
            let mut game = TicTacToe::new();
            let player = StrategyPlayer::new('X', chromosome.clone());
            let opponent = RandomComputerPlayer::new('O');
            while game.available_moves().len() > 0 {
                let square = if game.available_moves().len() % 2 == 0 {
                    player.get_move(&game)
                } else {
                    opponent.get_move(&game)
                };
                game.make_move(square, if game.available_moves().len() % 2 == 0 { 'X' } else { 'O' });

                println!("=====================");
                game.print_board();

                if let Some(winner) = game.current_winner {
                    println!("Winner: {}", winner);
                    println!("chromosome: {:?}", chromosome);
                    if winner == 'X' {
                        wins += 1;
                    }
                    break;
                }
            }
        }
        wins
    }

    pub fn selection(&self) -> Vec<Chromosome> {
        // Selects the best half of the chromosomes from the population.
        // The best chromosomes are selected based on their fitness.
        let mut selected = Vec::new();
        for _ in 0..self.population_size / 2 {
            let i = rand::thread_rng().gen_range(0..self.population.len());
            let j = rand::thread_rng().gen_range(0..self.population.len());
            let fitness_i = self.fitness(&self.population[i]);
            let fitness_j = self.fitness(&self.population[j]);
            if fitness_i > fitness_j {
                selected.push(self.population[i].clone());
            } else {
                selected.push(self.population[j].clone());
            }
        }
        selected
    }

    pub fn crossover(&self, parent1: &Chromosome, parent2: &Chromosome) -> Chromosome {
        // Combines two chromosomes to create a new chromosome.
        if rand::thread_rng().gen_bool(self.crossover_rate) {
            let point = rand::thread_rng().gen_range(1..parent1.len());
            let mut child = parent1[..point].to_vec();
            child.extend_from_slice(&parent2[point..]);
            child
        } else {
            parent1.clone()
        }
    }

    pub fn mutate(&self, chromosome: &mut Chromosome) {
        // Mutates a given chromosome in-place.
        if rand::thread_rng().gen_bool(self.mutation_rate) {
            let point = rand::thread_rng().gen_range(0..chromosome.len());
            chromosome[point] = rand::thread_rng().gen_range(0..9);
        }
    }

    pub fn evolve(&mut self) {
        // Evolves the population of chromosomes.
        for generation in 0..self.generations {
            let selected = self.selection();
            let mut next_generation = Vec::new();
            for i in (0..selected.len()).step_by(2) {
                let parent1 = &selected[i];
                let parent2 = &selected[i + 1];
                let mut child1 = self.crossover(parent1, parent2);
                let mut child2 = self.crossover(parent2, parent1);
                self.mutate(&mut child1);
                self.mutate(&mut child2);
                next_generation.push(child1);
                next_generation.push(child2);
            }
            self.population = next_generation;
            let best_fitness = self.population.iter().map(|chrom| self.fitness(chrom)).max().unwrap();
            println!("Generation {}: Best Fitness = {}", generation + 1, best_fitness);
        }
    }
}
