use neatcore::Core;
use network::NeuralNetwork;

const POPULATION: usize = 2;

fn main() {    
    let mut core = Core::init(
        POPULATION, // # of genomes
        None,
        None,
        (0, 1), /* # of inputs, # of outputs, (outputs cant be 0) */
        fitness_function
    );

    core.train();
}

fn fitness_function(_network: NeuralNetwork) -> f64 {
    1.0
}