mod components;

use components::*;
use innovation::*;

use std::collections::{HashMap, HashSet};

type Layers = Vec<Vec<Vec<usize>>>;
pub struct NeuralNetwork {
    neurons: Vec<Neuron>,
    connectors: Vec<Connector>,
    connector_map: HashMap<(usize, usize), usize>,
    layers: Layers,
}

impl NeuralNetwork {
    fn new() -> NeuralNetwork {
        NeuralNetwork {
            neurons: Vec::new(),
            connectors: Vec::new(),
            connector_map: HashMap::new(),
            layers: Vec::new(),
        }
    }

    pub fn init(genome: GenomeType, innovation_table: &InnovationTable) {
        let mut network = NeuralNetwork::new();
        let mut neurons: Vec<usize> = Vec::new();

        for i in 0..genome.0.len() {

            // If gene is false, it disables it
            if !genome.2[i] {
                continue;
            }

            let innovation = &innovation_table.innovations[genome.0[i]];

            let connector = Connector {
                from: innovation.from,
                to: innovation.to,
                weight: genome.1[i],
            };

            neurons.push(connector.from);
            neurons.push(connector.to);

            network.add_connector(connector);
        }


        // Removes duplicate neurons
        let mut seen_neurons = HashSet::new();
        neurons.retain(|x| seen_neurons.insert(*x));

        for neuron_id in neurons {
            let neuron = Neuron {
                id: neuron_id,
                from_arr: Vec::new(),
                to_arr: Vec::new(),
                kind: innovation_table.get_levels(neuron_id),
                value: 0.0,
            };

            network.add_neuron(neuron);
        }

        // Now we get the layers
        /*
            layers = 
            [ List of "parts" of the network
                [ List of sublayers
                    [],
                    [],
                    [],
                ]

                [...],
                [...],
            ]
         */

        let mut layers: Layers = Vec::new();

    }

    // ! Eats connector
    fn add_connector(&mut self, connector: Connector) {
        self.connector_map.insert((connector.from, connector.to), self.connectors.len());
        self.connectors.push(connector);
    }

    // ! Eats neuron
    fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }
}