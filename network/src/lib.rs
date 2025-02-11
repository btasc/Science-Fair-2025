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
        }
    }

    pub fn init(genome: GenomeType, innovation_table: &InnovationTable) -> NeuralNetwork {
        let mut network = NeuralNetwork::new();
        let mut neurons: Vec<usize> = Vec::new();

        network.neuron_levels = (innovation_table.neuron_levels.0.clone(), innovation_table.neuron_levels.0.clone());

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
                    to_connections.extend(network.get_neuron(neuron).to_arr.clone());
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
        network
    }

    fn fire_connector(&mut self, connector: usize) {
        let Connector { from, to, weight, .. } = self.connectors[connector];

        let mut lower: usize = 0;
        let mut higher: usize = 0;

        match from > to {
            true => {
                lower = *self.neuron_map.get(&to).unwrap();
                higher = *self.neuron_map.get(&from).unwrap();
            }
            false => {
                lower = *self.neuron_map.get(&from).unwrap();
                higher = *self.neuron_map.get(&to).unwrap();
            }
        }

        let (lower_arr, higher_arr) = self.neurons.split_at_mut(lower + 1);

        let neuron_1 = &mut lower_arr[lower];
        let neuron_2 = &mut higher_arr[higher - lower - 1];

        neuron_2.value += neuron_1.value * weight;
    }

    fn prepare_inputs(&mut self, inputs: Vec<f64>) {
        #[cfg(debug_assertions)]
        {
            if inputs.len() != self.neuron_levels.0.len() {
                panic!("Inputs are not equal to the number of input neurons");
            }
        }

        for neuron in &mut self.neurons {
            neuron.value = 0.0;
        }

        for (i, input_neuron) in self.neuron_levels.0.iter().enumerate() {
            self.neurons[*self.neuron_map.get(input_neuron).unwrap()].value = inputs[i];
        }
    }

    fn get_order(&self) -> Vec<Vec<usize>> {
        let mut order: Vec<Vec<usize>> = Vec::new();

        for (i, component) in self.layers.iter().enumerate() {
            for (j, layer) in component.iter().enumerate() {
                order.push(Vec::new());

                for (k, neuron) in layer.iter().enumerate() {
                    let to_arr = self.get_neuron(neuron).to_arr.clone();

                    order[j].extend(to_arr);
                }
            }
        }

        order
    }

    pub fn run(&mut self, inputs: Vec<f64>) {
        self.prepare_inputs(inputs);
        
        let order = self.get_order();
        println!("{:?}", order);
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