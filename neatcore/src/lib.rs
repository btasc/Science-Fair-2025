use network::NeuralNetwork;
use innovation::{InnovationTable, RawInnovation};

use std::collections::HashSet;

pub type GenomeType = (Vec<usize>, Vec<f64>, Vec<bool>);

const C1: f64 = 1.0;
const C2: f64 = 1.0;
const C3: f64 = 1.0;

use rand::Rng;

// Odds for mutation
const ADD_NODE: f64 = 0.04; // 4%
const ADD_CONN: f64 = 0.08; // 8%
const CHNG_WEIGHT: f64 = 0.12; // 12%

pub struct Core {
    gen_arr: Vec<GenomeType>,
    table: InnovationTable,
}

impl Core {
    pub fn new() -> Core {
        Core {
            gen_arr: Vec::new(),
            table: InnovationTable::new(),
        }
    }

    pub fn init_table(&mut self, levels: (Vec<usize>, Vec<usize>), innovations: Vec<RawInnovation>) {
        #[cfg(debug_assertions)]
        {
            for output in &levels.1 {
                if levels.0.contains(output) {
                    panic!("Output neuron cannot be in input layer at neatcore");
                }
            }
        }

        self.table.set_levels(levels.0, levels.1);

        for innovation in innovations {
            self.table.add_innovation(innovation);
        }
    }

    pub fn init_genome(&mut self, population: usize, genome: GenomeType) {
        #[cfg(debug_assertions)]
        {
            if genome.0.len() != genome.1.len() || genome.0.len() != genome.2.len() || genome.1.len() != genome.2.len() {
                panic!("Genome length mismatch at neatcore");
            }
        }

        for _ in 0..population {
            self.gen_arr.push(genome.clone());
        }
    }

    pub fn run(&self, index: usize, inputs: Vec<f64>) -> Vec<f64> {
        #[cfg(debug_assertions)]
        {
            if inputs.len() != self.table.neuron_levels.0.len() {
                panic!("Input length does not match input layer length at neatcore");
            }
        }

        let genome = self.gen_arr[index].clone();

        let mut network = NeuralNetwork::init(genome, &self.table);
        network.run(inputs)
    }

    pub fn compare(&self, index1: usize, index2: usize) -> f64 {
        let genome1 = &self.gen_arr[index1];
        let genome2 = &self.gen_arr[index2];

        let mut set1: HashSet<usize> = genome1.0.iter().cloned().collect();
        let mut set2: HashSet<usize> = genome2.0.iter().cloned().collect();

        #[cfg(debug_assertions)]
        {
            if set1.len() != genome1.0.len() || set2.len() != genome2.0.len() {
                panic!("Duplicate gene at neatcore");
            }
        }

        let mut matching: usize = 0;
        let mut average_dif: f64 = 0.0;

        genome1.0
            .iter()
            .zip(genome2.0.iter())
            .enumerate()
            .for_each(|(i, (a, b))| 
                if a == b { 
                    matching += 1;
                    average_dif += (genome1.1[i] - genome2.1[i]).abs();
                    set1.remove(a);
                    set2.remove(a);
                }
            );
        
        average_dif /= matching as f64;

        let disjoint = set1.intersection(&set2).count();
        let excess = set1.difference(&set2).count();

        (
            (C1 * excess as f64) + 
            (C2 * disjoint as f64)
        ) / (std::cmp::max(genome1.0.len(), genome2.0.len()) as f64) + 
        (C3 * average_dif as f64)
    }

    pub fn mutate(&self, index: usize) {
        let genome = &self.gen_arr[index];

        Self::mutate_net(NeuralNetwork::init(genome.clone(), &self.table));
    }

    fn mutate_net(network: NeuralNetwork) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let random_tup: (f64, f64, f64) = (rng.gen(), rng.gen(), rng.gen());
    
        println!("{:?}", random_tup);
        network.get_random_connection();
    
        // Add new node
        if ADD_CONN > random_tup.0 {
            
        }
    }
}