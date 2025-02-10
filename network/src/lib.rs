mod components;

use components::*;
use innovation::*;

use std::collections::{HashMap, HashSet};

type Layers = Vec<Vec<Vec<usize>>>;
pub struct NeuralNetwork {
    neurons: Vec<Neuron>,
    neuron_map: HashMap<usize, usize>,
    connectors: Vec<Connector>,
    connector_map: HashMap<(usize, usize), usize>,
    layers: Layers,
}

impl NeuralNetwork {
    fn new() -> NeuralNetwork {
        NeuralNetwork {
            neurons: Vec::new(),
            neuron_map: HashMap::new(),
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

                // Just for layers
                calls: 0,
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

            || <- to
            []
            || <- from
         */
        for i in 0..network.connectors.len() {
            network.neurons[*network.neuron_map.get(&network.connectors[i].from).unwrap()].to_arr.push(i);
            network.neurons[*network.neuron_map.get(&network.connectors[i].to).unwrap()].from_arr.push(i);

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

        let mut layers: Layers = vec![Vec::new()];
        let mut queue: Vec<usize> = innovation_table.neuron_levels.0.clone();

        while queue.len() > 0 {
            let mut temp_queue: Vec<usize> = Vec::new();

            for queue_neuron_id in &queue {
                let queue_neuron_to_arr = network.neurons[*network.neuron_map.get(queue_neuron_id).unwrap()].to_arr.clone();

                for connector_id in queue_neuron_to_arr {
                    let connector_to_id = network.connectors[connector_id].to;

                    temp_queue.push(connector_to_id);
                    network.neurons[*network.neuron_map.get(&connector_to_id).unwrap()].calls += 1;
                }
            }

            // temp queue should now have all neurons that connect to queue, and all those neurons should have their calls incremented
            // we can now check if all the neurons in temp have been called the same amount of times as they have froms
            layers[0].push(queue.clone());
            queue = Vec::new();

            for neuron_id in temp_queue.clone() {
                let calls = network.neurons[*network.neuron_map.get(&neuron_id).unwrap()].calls;
                let froms = network.neurons[*network.neuron_map.get(&neuron_id).unwrap()].from_arr.len();

                if calls == froms {
                    queue.push(neuron_id);
                }
            }
        }

        println!("{:?}", layers);
    }

    // ! Eats connector
    fn add_connector(&mut self, connector: Connector) {
        self.connector_map.insert((connector.from, connector.to), self.connectors.len());
        self.connectors.push(connector);
    }

    // ! Eats neuron
    fn add_neuron(&mut self, neuron: Neuron) {
        self.neuron_map.insert(neuron.id, self.neurons.len());
        self.neurons.push(neuron);
    }
}