use network::{NeuralNetwork, Layers, Genome};
use innovation::{InnovationTable, RawInnovation, Type};
use timer::Timer;

use rand::Rng;
use serde::Serialize;

use std::{collections::HashSet};


// Compare
    const C1: f64 = 1.0;
    const C2: f64 = 0.5;
    const C3: f64 = 0.5;

// Mutation
    const WGHT_CHNG_RNG: (f64, f64) = (0.25, -0.25);

    // Odds
        const ADD_NODE: f64 = 1.0;//0.04; // 4%
        const ADD_CONN: f64 = 1.0;//0.08; // 8%
        const CHNG_WEIGHT: f64 = 1.0;//0.12; // 12%

// Backpropogation
const LRN_RATE: f64 = 0.01;

struct Species {
    exemplar: Genome,
    members: Vec<usize>,
    stagnant_generations: usize,
    fitness: f64,
}

#[derive(Serialize, Debug)]
struct JSON_network {
    nodes: Vec<usize>,
    layers: Vec<Vec<usize>>,
    connections: Vec<(usize, usize)>,
}

pub struct Core {
    population: usize,
    gen_arr: Vec<Genome>,
    fit_arr: Vec<f64>,
    table: InnovationTable,
    output_set: HashSet<usize>,
    species: Vec<Species>,
}

impl Species {
    fn init(exemplar: Genome,) -> Self {
        Self {
            exemplar,
            members: Vec::new(),
            stagnant_generations: 0,
            fitness: f64::NAN,
        }
    }
}

impl Core {
    fn new() -> Self {
        Self {
            population: 0,
            gen_arr: Vec::new(),
            fit_arr: Vec::new(),
            table: InnovationTable::new(),
            output_set: HashSet::new(),
            species: Vec::new(),
        }
    }

    pub fn init(
        population: usize, 
        default_genome: Option<&Genome>, 
        innovations: Option<Vec<(usize, usize)>>, // No Type becuase innovation isnt imported in main 
        levels: (usize, usize),
    ) -> Self {
        let mut core = Core::new();

        core.population = population;

        match default_genome {
            Some(genome) => {
                #[cfg(debug_assertions)]
                {
                    if genome.0.len() != genome.1.len() || genome.0.len() != genome.2.len() {
                        panic!("Genome must have equal length vectors at neatcore");
                    }
                }

                for _ in 0..population {

                    core.gen_arr.push((*genome).clone());
                    core.fit_arr.push(0.0);
                }
            },
            None => {
                for _ in 0..population {
                    core.gen_arr.push(Genome::new());
                    core.fit_arr.push(0.0);
                }
            }
        }

        if levels.1 == 0 {
            panic!("Cannot set levels.1 to 0 at neatcore");
        }

        let level0_range = 1..(levels.0 + 1);
        let level1_range = (levels.0 + 1)..(levels.1 + 2);
        
        #[cfg(debug_assertions)]
        {
            for output in level1_range.clone() {
                if level0_range.contains(&output) {
                    panic!("Output neuron cannot be in input layer at neatcore");
                }
            }
        }

        core.output_set.extend(level0_range.clone());
        core.output_set.extend(level1_range.clone());

        core.table.set_levels(level0_range.collect::<Vec<usize>>(), level1_range.collect::<Vec<usize>>());

        match innovations {
            Some(innovations) => {
                for innovation in innovations {
                    core.table.add_innovation((innovation.0, innovation.1, Type::Connector));
                }
            },
            None => (),
        }
    

        core
    }

    fn run(&self, index: usize, inputs: Vec<f64>) -> Vec<f64> {
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

    fn mutate(&mut self, index: usize) {
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
                weights[rng.gen_range(0..len)] += change;
            }
        }
    }

    fn get_random_connector(&self, index: usize) -> (usize, usize) {
        let genome = &self.gen_arr[index];
        let network = NeuralNetwork::init(genome, &self.table);

        let all_connections = Self::get_all_connections(&network.layers, &network.neuron_levels);

        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        assert_ne!(all_connections.len(), 0, "All connections of a network are equal to 0. This should not be possible as bias neuron and output should always be able to connect");
        all_connections[rng.gen_range(0..all_connections.len())]
    }

    fn add_connector(&mut self, index: usize, connection: (usize, usize)) {
        let genome = &mut self.gen_arr[index];

        match self.table.get_innovation((connection.0, connection.1, Type::Connector)) {
            Some(id) => {
                match genome.0.iter().position(|x| x == id) {
                    Some(innov_index) => {
                        genome.2[innov_index] = !genome.2[innov_index];
                    },
                    None => {
                        genome.0.push(*id);
                        genome.1.push(0.0);
                        genome.2.push(true);
                    }
                } 
            },
            None => {
                self.table.add_innovation((connection.0, connection.1, Type::Connector));

                genome.0.push(self.table.innovations.len() - 1);
                genome.1.push(0.0);
                genome.2.push(true);
            }
        }
     }

    fn get_all_connections(layers: &Layers, levels: &(Vec<usize>, Vec<usize>)) -> Vec<(usize, usize)> {
        let mut possible_connections: Vec<(usize, usize)> = Vec::new();

        let input_hash: HashSet<usize> = levels.0.iter().copied().collect();
        let output_hash: HashSet<usize> = levels.1.iter().copied().collect();

        for (i, layer) in layers.iter().enumerate() {
            for from_neuron in layer {
                if output_hash.contains(from_neuron) {
                    continue;
                }

                for to_layer in layers[i+1..layers.len()].iter() {
                    for to_neuron in to_layer {
                        if  input_hash.contains(to_neuron) {
                            continue;
                        }

                        possible_connections.push((*from_neuron, *to_neuron));
                    }
                }
            }
        }
        
        possible_connections
    }

    pub fn to_json(&self, index: usize, path: &str) {
        let network = NeuralNetwork::init(&self.gen_arr[index], &self.table);

        let mut nodes: Vec<usize> = Vec::new();
        let mut connections: Vec<(usize, usize)> = Vec::new();

        for neuron in network.neurons {
            nodes.push(neuron.id);
        }

        for connection in network.connectors {
            connections.push((connection.from, connection.to));
        }

        let json_net = JSON_network {
            nodes,
            connections,
            layers: network.layers,
        };

        let serialized = serde_json::to_string(&json_net).unwrap();
        std::fs::write(path, serialized).unwrap();
    }

    fn backprop(&mut self, index: usize, breadth: usize, range: usize, itterations: usize) {
        let network = NeuralNetwork::init(&self.gen_arr[index], &self.table);

        let inc = range as f64 / breadth as f64;

        log(&network.layers);
        
        for _ in 0..itterations {
            for b in (breadth as i32 * -1)..(breadth as i32) {
                let input = inc * b as f64;
    
                
            }
        }
    }

    

    pub fn train(&mut self) {
        // mutate population
        self.table.add_innovation((1, 2, Type::Connector));

        self.gen_arr[0].0 = vec![0];
        self.gen_arr[0].1 = vec![0.0];
        self.gen_arr[0].2 = vec![true];

        let mut timer = Timer::new();
        timer.start();

        self.backprop(0, 10, 5, 10);

        timer.stop();
        timer.log();


        //self.to_json(0, "../render/network.json");
    }
}

fn log<T>(var: &T) where T: std::fmt::Debug {
    println!("{:?}", var);
}