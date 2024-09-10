mod tic_tac_toe;
mod player;
mod genetic_algorithm;

use genetic_algorithm::GeneticAlgorithm;

fn main() {
    println!("===============genetic-tac-toe=================");
    let mut ga = GeneticAlgorithm::new(
        100, 
        0.01, 
        0.7, 
        50
    );
    ga.evolve();
}
