use neatcore::Core;
use network::{NeuralNetwork};

const POPULATION: usize = 150;

fn main() {    
    let mut core = Core::init(
        POPULATION, // # of genomes
        None, // None = (Vec::new(), Vec::new(), Vec::new())
        None, // None = No starting innovations
        (vec![1], vec![2]), // [input ids], [output ids]. Cant be 0 as 0 is bias and is always there
        fitness_function
    ); 

    core.train();
}

fn fitness_function(network: NeuralNetwork) -> f64 {
    1.0
}