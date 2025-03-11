use crate::NeuralNetwork;
use std::collections::HashSet;

pub fn layer_network(network: &mut NeuralNetwork) -> Vec<Vec<usize>> {
    let mut layers: Vec<Vec<usize>> = Vec::new();
    let mut queue: Vec<usize> = Vec::new();

    let mut input_hash: HashSet<usize> = network.neuron_levels.0.iter().copied().collect();
    input_hash.insert(0); // Bias is always input

    let input_hash = input_hash;

    layers.push(
        input_hash.iter().copied().collect::<Vec<usize>>()
    );

    // Layers now has layer 0 set to inputs
    for neuron in network.neurons.iter() {
        if neuron.from_arr.is_empty() && !input_hash.contains(&neuron.id) {
            queue.push(neuron.id);
        }
    }

    while !queue.is_empty() {
        let connectors: Vec<usize> = queue.iter()
            .flat_map(|neuron_id| network.get_neuron(neuron_id).to_arr.clone())
            .collect();

        // Set of neurons that arent verified to be in the next layer, so temp
        let temp_neurons = connectors.into_iter()
            .map(|connector_id|{
                let neuron = network.connectors[connector_id].to;

                network.neurons[
                    *network.neuron_map.get(&neuron).unwrap()
                ].calls += 1;

                neuron
            })
            .collect::<Vec<usize>>();



    }

    layers
}