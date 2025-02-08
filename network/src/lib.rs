mod components;

use components::*;
use innovation::*;
use std::collections::HashMap;

pub struct NeuralNetwork {
    neurons: Vec<Neuron>,
    connectors: Vec<Connector>,
    connector_map: HashMap<(i32, i32), i32>,
    layers: Vec<Vec<Vec<i32>>>,
}

impl NeuralNetwork {
    pub fn new(genome: genome_type, innovation_table: &InnovationTable) -> NeuralNetwork {
        NeuralNetwork {
            neurons: Vec::new(),
            connectors: Vec::new(),
            connector_map: HashMap::new(),
            layers: Vec::new(),
        }
    }
}