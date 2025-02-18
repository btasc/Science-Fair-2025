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
    neuron_levels: (Vec<usize>, Vec<usize>),
    order: Vec<Vec<usize>>,
}


impl NeuralNetwork {
    fn new() -> NeuralNetwork {
        NeuralNetwork {
            neurons: Vec::new(),
            neuron_map: HashMap::new(),
            connectors: Vec::new(),
            connector_map: HashMap::new(),
            layers: Vec::new(),
            neuron_levels: (Vec::new(), Vec::new()),
            order: Vec::new(),
        }
    }

    pub fn init(genome: GenomeType, innovation_table: &InnovationTable) -> NeuralNetwork {
        let mut network = NeuralNetwork::new();
        let mut neurons: Vec<usize> = Vec::new();

        network.neuron_levels = (innovation_table.neuron_levels.0.clone(), innovation_table.neuron_levels.1.clone());

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

        neurons.push(0);
        neurons.extend(&innovation_table.neuron_levels.0);
        neurons.extend(&innovation_table.neuron_levels.1);

        // Removes duplicate neurons
        let mut seen_neurons = HashSet::new();
        neurons.retain(|x| seen_neurons.insert(*x));

        for neuron_id in neurons {
            let neuron = Neuron {
                id: neuron_id,
                from_arr: Vec::new(),
                to_arr: Vec::new(),
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
        
        #[cfg(debug_assertions)]
        {
            for neuron in &network.neurons {
                if innovation_table.neuron_levels.0.contains(&neuron.id) {
                    assert_eq!(neuron.from_arr.len(), 0, "Input neurons should not have any connectors going from them");
                }
            }
        }
        
        // Now we get the layers
        /*
            layers = [ Components [ Layers [ Sublayers [ Neurons, ... ], ... ], ... ], ... ]
            See MD for more explanation
        */
            
        let mut component_queue: Vec<usize> = Vec::new();

        // Fill the component queue with non dependant neurons
        for neuron in &network.neurons {
            if neuron.from_arr.len() == 0 {
                component_queue.push(neuron.id);
            }
        }

        let mut layers: Layers = Vec::new();

        // Now we get the layers
        for component in component_queue {
            let mut component_sublayers: Vec<Vec<usize>> = Vec::new();
            let mut queue: Vec<usize> = vec![component];

            while queue.len() > 0 {
                let mut to_connections: Vec<usize> = Vec::new();

                for neuron in &queue {
                    to_connections.extend(&network.get_neuron(neuron).to_arr);
                }

                #[cfg(debug_assertions)]
                {
                    let mut set = HashSet::new();
                    for neuron in &to_connections {
                        assert!(set.insert(*neuron), "Duplicate neuron in to_connections");
                    }
                }

                let mut temp_neurons: Vec<usize> = Vec::new();

                // Get all the to neurons in the connections array and increment calls
                for connection in to_connections {
                    let to_neuron = network.connectors[connection].to;

                    temp_neurons.push(to_neuron);
                    network.neurons[*network.neuron_map.get(&to_neuron).unwrap()].calls += 1;
                }

                component_sublayers.push(queue); // Doesnt eat queue
                queue = Vec::new();

                for neuron in temp_neurons {
                    let Neuron { calls, from_arr, .. } = network.get_neuron(&neuron);

                    if *calls == from_arr.len() {
                        queue.push(neuron);
                    }

                    #[cfg(debug_assertions)]
                    {
                        if *calls > from_arr.len() {
                            panic!("Calls are greater than from_arr.len");
                        }
                    }
                }
            }

            layers.push(component_sublayers);
        }

        network.layers = layers;
        network.order = network.get_order();
        network
    }

    fn fire_connector(&mut self, connector: usize) {
        let Connector { from, to, weight, .. } = self.connectors[connector];

        let neuron_1_value: f64 = self.get_neuron(&from).value;

        let neuron_2 = &mut self.neurons[*self.neuron_map.get(&to).unwrap()];
        neuron_2.value += neuron_1_value * weight;
    }

    fn prepare_inputs(&mut self, inputs: Vec<f64>) {
        #[cfg(debug_assertions)]
        {
            if inputs.len() != self.neuron_levels.0.len() {
                panic!("Inputs are not equal to the number of input neurons");
            }
        }

        for neuron in &mut self.neurons {
            if neuron.id == 0 {
                neuron.value = 1.0;
            } else {
                neuron.value = 0.0;
            }
        }

        for (i, input_neuron) in self.neuron_levels.0.iter().enumerate() {
            self.neurons[*self.neuron_map.get(input_neuron).unwrap()].value = inputs[i];
        }
    }

    fn get_order(&self) -> Vec<Vec<usize>> {
        let mut neuron_order: Vec<Vec<usize>> = Vec::new();
        //[[2, 5, 1, 7], [6, 4], [3]];
        let mut longest_layer: usize = 0;

        for component in &self.layers {
            if component.len() > longest_layer {
                longest_layer = component.len();
            }
        }
        
        for i in 0..longest_layer {
            neuron_order.push(Vec::new());
            for component in &self.layers {
                if component.len() > i {
                    neuron_order[i].extend(&component[i]);
                }
            }
        }

        let mut order: Vec<Vec<usize>> = Vec::new();

        for (i, layer) in neuron_order.iter().enumerate() {
            order.push(Vec::new());

            for neuron in layer {
                order[i].extend(&self.get_neuron(neuron).to_arr);
            }
        }

        #[cfg(debug_assertions)]
        {
            let mut length = 0;

            for layer in order.iter() {
                length += layer.len();
            }

            if length != self.connectors.len() {
                panic!("Order length does not match connector length");
            }
        }

        order
    }

    pub fn run(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        self.prepare_inputs(inputs);
        
        let order = self.order.clone();

        for set in order {
            for connector in set {
                self.fire_connector(connector);
            }
        }

        let mut output: Vec<f64> = Vec::new();

        for output_neuron in &self.neuron_levels.1 {
            output.push(
                self.get_neuron(output_neuron).value
            );
        }

        output
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

    fn get_neuron(&self, id: &usize) -> &Neuron {
        &self.neurons[*self.neuron_map.get(id).unwrap()]
    }
}