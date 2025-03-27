use network::{NeuralNetwork, Layers, Genome};
use innovation::{InnovationTable, RawInnovation, Type};

use rand::Rng;
use serde::Serialize;

use std::collections::HashSet;


const C1: f64 = 1.0;
const C2: f64 = 0.5;
const C3: f64 = 0.4;

const WGHT_CHNG_RNG: (f64, f64) = (0.25, -0.25);

// Odds for mutation
const ADD_NODE: f64 = 1.0;//0.04; // 4%
const ADD_CONN: f64 = 1.0;//0.08; // 8%
const CHNG_WEIGHT: f64 = 1.0;//0.12; // 12%

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
    fitness_function: Option<fn(NeuralNetwork) -> f64>,
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
            fitness_function: None,
            species: Vec::new(),
        }
    }

    pub fn init(
        population: usize, 
        default_genome: Option<&Genome>, 
        innovations: Option<Vec<(usize, usize)>>, // No Type becuase innovation isnt imported in main 
        levels: (Vec<usize>, Vec<usize>), 
        fitness_function: fn(NeuralNetwork) -> f64
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
                    core.table.add_innovation((innovation.0, innovation.1, Type::Connector));
                }
            },
            None => (),
        }
        
        core.fitness_function = Some(fitness_function);

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

    fn compare(&self, index1: usize, index2: usize) -> f64 {
        let (genome1, genome2) = if self.gen_arr[index1].0.len() > self.gen_arr[index2].0.len() {
            (&self.gen_arr[index1], &self.gen_arr[index2])
        } else {
            (&self.gen_arr[index1], &self.gen_arr[index2])
        };

        // Genome 1 is the longer one

        
        let mut matching: usize = 0;
        let mut disjoint: usize = 0;
        let mut excess:   usize = 0;
        
        let mut weight_sum: f64 = 0.0;

        genome2.0.iter()
            .for_each(|&id| {
                match genome1.find_weight(id) {
                    Some(weight) => {
                        weight_sum += weight;
                        matching += 1;
                    },
                    None => (),
                }
            });

        let mut average_weight_dif = 0.0;

        if matching != 0 {
            average_weight_dif = weight_sum / matching as f64;
        }

        let longest_f64 = genome1.0.len() as f64;

        
        

        (C1 * excess as f64 / longest_f64) + (C2 * disjoint as f64 / longest_f64) + (C3 * average_weight_dif as f64)
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

    fn get_fitness(&self, index: usize) -> f64 {
        let genome: &Genome = &self.gen_arr[index];
        let network = NeuralNetwork::init(genome, &self.table);

        (self.fitness_function.unwrap())(network)
    }

    fn get_all_fitness(&mut self) {
        for i in 0..self.population {
            self.fit_arr[i] = self.get_fitness(i);
        }
    }

    fn crossover(&self, index1: usize, index2: usize) -> Genome {
        if index1 > self.population || index2 > self.population {
            panic!("Cant crossover genome index that is greater than population. Index1: {}, Index2: {}, Population: {}", index1, index2, self.population);
        }

        let genome1: &Genome = &self.gen_arr[index1];
        let genome2: &Genome = &self.gen_arr[index2];

        let new_genome: Genome = Genome::new();

        new_genome
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

    pub fn train(&mut self) {
        // mutate population
        for i in 0..3 {
            self.mutate(0);
        }

        //self.to_json(0, "../render/network.json");

        /*

        for genome in &self.gen_arr {
            let set: HashSet<usize> = genome.0.iter().copied().collect(); 
            if set.len() != genome.0.len() {
                panic!("{:?}", genome.0);
            }
        }

        // Organize it into species
        'outer: for index in 0..self.population {
            for specie in &mut self.species {
                let distance = Self::compare(&self.gen_arr[index], &specie.exemplar);
                if distance < COMPATABILITY_THRESHOLD {
                    specie.members.push(index);
                    continue 'outer;
                }
            }

            self.species.push(
                Species::init(self.gen_arr[index].clone())
            );

            let len = self.species.len();
            self.species[len-1].members.push(index);
        }        

        self.get_all_fitness();

        let mut average_species_fitness: f64 = 0.0;

        for specie in &mut self.species {
            specie.fitness = specie.members.iter()
                .map(|&index| self.fit_arr[index] / specie.members.len() as f64)
                .sum();

            average_species_fitness += specie.fitness;
        }

        let mut network_budget: Vec<usize> = Vec::new();

        */
    }
}

fn log<T>(var: &T) where T: std::fmt::Debug {
    println!("{:?}", var);
}