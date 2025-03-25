mod components;
mod layering;

use components::*;

use layering::layer_network;
use innovation::{InnovationTable, Type};

use std::collections::{HashMap, HashSet};
use std::thread;
use std::time;

pub type Layers = Vec<Vec<usize>>;

pub struct NeuralNetwork {
    pub neurons: Vec<Neuron>,
    neuron_map: HashMap<usize, usize>,
    pub connectors: Vec<Connector>,
    pub connector_map: HashMap<(usize, usize), usize>,
    pub layers: Layers,
    pub neuron_levels: (Vec<usize>, Vec<usize>),
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

    pub fn init(genome: &GenomeType, innovation_table: &InnovationTable) -> NeuralNetwork {
        let mut network = NeuralNetwork::new();
        let mut neurons: Vec<usize> = Vec::new();

        network.neuron_levels = (innovation_table.neuron_levels.0.clone(), innovation_table.neuron_levels.1.clone());
        
        for i in 0..genome.0.len() {

            let innovation = &innovation_table.innovations[genome.0[i]];
            // An error here means that theres a gene in a genome that doesnt exist in the innov table
            // Probably something custom like using gene 14 or something

            if innovation.kind == Type::Neuron {
                continue;
            }

            let mut weight = 0.0;

            if genome.2[i] {
                weight = genome.1[i];
            }

            let connector = Connector {
                from: innovation.from,
                to: innovation.to,
                weight,
                id: innovation.id,
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

        network.layers = layer_network(&mut network);
        
        network.layers = network.layers;
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

    fn get_order(&self) -> Layers {
        let mut order: Layers = Vec::new();

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
        self.connector_map.insert((connector.from, connector.to), connector.id);
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