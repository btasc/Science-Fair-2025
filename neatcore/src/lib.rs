use network::{NeuralNetwork, Layers};
use innovation::{InnovationTable, RawInnovation, Type};

use std::collections::HashSet;

pub type GenomeType = (Vec<usize>, Vec<f64>, Vec<bool>);

const C1: f64 = 1.0;
const C2: f64 = 1.0;
const C3: f64 = 1.0;

const WGHT_CHNG_RNG: (f64, f64) = (0.25, -0.25);

use rand::Rng;

// Odds for mutation
const ADD_NODE: f64 = 0.04; // 4%
const ADD_CONN: f64 = 0.08; // 8%
const CHNG_WEIGHT: f64 = 0.12; // 12%

pub struct Core {
    pub gen_arr: Vec<GenomeType>,
    table: InnovationTable,
    output_set: HashSet<usize>,
}

impl Core {
    fn new() -> Core {
        Core {
            gen_arr: Vec::new(),
            table: InnovationTable::new(),
            output_set: HashSet::new(),
        }
    }

    pub fn init(population: usize, default_genome: Option<GenomeType>, innovations: Option<Vec<RawInnovation>>, levels: (Vec<usize>, Vec<usize>)) -> Self {
        let mut core = Core::new();

        match default_genome {
            Some(genome) => {
                #[cfg(debug_assertions)]
                {
                    if genome.0.len() != genome.1.len() || genome.0.len() != genome.2.len() || genome.1.len() != genome.2.len() {
                        panic!("genome length mismatch at neatcore");
                    }
                }
                
                for _ in 0..population {
                    core.gen_arr.push(genome.clone());
                }
            },
            None => {
                for _ in 0..population {
                    core.gen_arr.push((Vec::new(), Vec::new(), Vec::new()));
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            for output in &levels.1 {
                if levels.0.contains(output) {
                    panic!("Output neuron cannot be in input layer at neatcore");
                }
            }
        }

        core.output_set.extend(levels.0.iter());
        core.output_set.extend(levels.1.iter());

        core.table.set_levels(levels.0, levels.1);

        match innovations {
            Some(innovations) => {
                for innovation in innovations {
                    core.table.add_innovation(innovation);
                }
            },
            None => (),
        }

        core
    }

    pub fn run(&self, index: usize, inputs: Vec<f64>) -> Vec<f64> {
        #[cfg(debug_assertions)]
        {
            if inputs.len() != self.table.neuron_levels.0.len() {
                panic!("Input length does not match input layer length at neatcore");
            }
        }

        let genome = &self.gen_arr[index];

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

    

    pub fn mutate(&mut self, index: usize) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let random_tup: (f64, f64, f64) = (rng.gen(), rng.gen(), rng.gen());

        if random_tup.0 < ADD_CONN {
            let chosen_connector = self.get_random_connector(index);
            self.add_connector(index, chosen_connector);
        }

        if random_tup.1 < ADD_NODE {
            let chosen_connector = self.get_random_connector(index);
            let new_neuron = self.table.inc_neuron();

            self.add_connector(index, (chosen_connector.0, new_neuron));
            self.add_connector(index, (new_neuron, chosen_connector.1));
        }

        if random_tup.2 < CHNG_WEIGHT {
            let weights = &mut self.gen_arr[index].1;
            let len = weights.len();

            if len != 0 {
                let change = rng.gen_range(WGHT_CHNG_RNG.1..WGHT_CHNG_RNG.0);
                (*weights)[rng.gen_range(0..len)] += change;
            }
        }
    }

    fn get_random_connector(&self, index: usize) -> (usize, usize) {
        let genome = &self.gen_arr[index];
        let network = NeuralNetwork::init(genome, &self.table);

        let all_connections = Self::get_all_connections(&network.layers, &network.neuron_levels);

        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        all_connections[rng.gen_range(0..all_connections.len())]
    }

    fn add_connector(&mut self, index: usize, connection: (usize, usize)) {
        let genome = &mut self.gen_arr[index];

        match self.table.get_innovation((connection.0, connection.1, Type::Connector)) {
            Some(id) => {
                match genome.0.iter().position(|x| x == id) {
                    Some(innov_index) => {
                        (*genome).2[innov_index] = !genome.2[innov_index];
                    },
                    None => {
                        (*genome).0.push(*id);
                        (*genome).1.push(0.0);
                        (*genome).2.push(true);
                    }
                } 
            },
            None => {
                self.table.add_innovation((connection.0, connection.1, Type::Connector));

                (*genome).0.push(self.table.innovations.len() - 1);
                (*genome).1.push(0.0);
                (*genome).2.push(true);
            }
        }
     }

    fn get_all_connections(layers: &Layers, levels: &(Vec<usize>, Vec<usize>)) -> Vec<(usize, usize)> {
        let mut possible_connections: Vec<(usize, usize)> = Vec::new();
    
        let mut flattend_possibilities: HashSet<&usize> = layers.iter()
            .flatten()
            .flatten()
            .collect();

        for input  in levels.0.iter() {
            flattend_possibilities.remove(input);
        }

        flattend_possibilities.remove(&0);

        // Remove mutability since its not needed
        let flattend_possibilities = flattend_possibilities;

        let output_hash: HashSet<&usize> = levels.1.iter().collect();

        for (component_index, component) in layers.iter().enumerate() {
            for (layer_index, layer) in component.iter().enumerate() {
                for from_neuron in layer {
                    if output_hash.contains(from_neuron) {
                        continue;
                    }

                    let mut possible_tos: HashSet<&usize> = flattend_possibilities.clone();

                    for layer in layers[component_index].iter().take(layer_index + 1) {
                        for banned_neuron in layer {
                            possible_tos.remove(banned_neuron);
                        }
                    }

                    for to_neuron in possible_tos.into_iter() {
                        possible_connections.push((*from_neuron, *to_neuron));
                    }
                }
            }
        }
    
        possible_connections
    }

    pub fn crossover(&self, index1: usize, index2: usize) -> GenomeType {
        if index1 == index2 {
            panic!("Cannot crossover genome with itself at neatcore");
        }

        let genome1 = &self.gen_arr[index1];
        let genome2 = &self.gen_arr[index2];

        let new_genome: GenomeType = (Vec::new(), Vec::new(), Vec::new());

        new_genome
    }
}

