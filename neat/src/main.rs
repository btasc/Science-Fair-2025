use neatcore::Core;
use network::*;
use innovation::*;

const POPULATION: usize = 1;

fn main() {    
    let mut core = Core::init(
        POPULATION, // # of genomes
        None,
        None,
        (1, 1), /* # of inputs, # of outputs, (outputs cant be 0) */
    );

    //core.train();
}

fn log<T>(var: &T) where T: std::fmt::Debug {
    println!("{:?}", var);
}