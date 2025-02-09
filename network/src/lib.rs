mod components;

use components::*;
use innovation::*;

use std::collections::{HashMap, HashSet};

type Layers = Vec<Vec<Vec<usize>>>;
pub struct NeuralNetwork {
    neurons: HashMap<usize, Neuron>,
    connectors: Vec<Connector>,
    connector_map: HashMap<(usize, usize), usize>,
    layers: Layers,
}

impl NeuralNetwork {
    fn new() -> NeuralNetwork {
        NeuralNetwork {
            neurons: HashMap::new(),
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
                kind: innovation_table.get_level(neuron_id),
                value: 0.0,
            };

            network.add_neuron(neuron);
        }

        // Assign the froms and tos to the neurons, so iterate through connectors and add them to the neurons
        /*
            [] <- Tos
            || <- Connector
            [] <- Froms


            so-
            [] <- neuron 2
            || <- connector 1
            [] <- neuron 1

            would be-
            neuron 1: tos = [connector 1]
            neuron 2: froms = [connector 1]

            tos = connectors that go to this neuron
            froms = connectors that go from this neuron
         */
        for i in 0..network.connectors.len() {
            network.neurons.get_mut(&network.connectors[i].from).unwrap().to_arr.push(i);
            network.neurons.get_mut(&network.connectors[i].to).unwrap().from_arr.push(i);
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
        self.neurons.insert(neuron.id, neuron);
    }
}