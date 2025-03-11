use neatcore::Core;
use network::NeuralNetwork;

const POPULATION: usize = 150;

fn main() {    
    let mut core = Core::init(
        POPULATION, // # of genomes
        (vec![0], vec![0.0], vec![false]),
        vec![(0, 1)], // Must have some values because mutation stuff
        (vec![1], vec![2]), // [input ids], [output ids]. Cant be 0 as 0 is bias and is always there
        fitness_function
    ); 

    core.train();
}

fn fitness_function(_network: NeuralNetwork) -> f64 {
    1.0
}