use crate::NeuralNetwork;
use std::collections::HashSet;

pub fn layer_network(network: &mut NeuralNetwork) -> Vec<Vec<usize>> {
    let mut layers: Vec<Vec<usize>> = Vec::new();
    let mut queue: Vec<usize> = Vec::new();

    let mut input_hash: HashSet<usize> = network.neuron_levels.0.iter().copied().collect();
    input_hash.insert(0); // Bias is always input

    let input_hash = input_hash; // Remove mutability

    layers.push(
        input_hash.iter().copied().collect::<Vec<usize>>()
    );

    // Layers now has layer 0 set to inputs
    for neuron in network.neurons.iter() {
        if neuron.from_arr.is_empty() {
            queue.push(neuron.id);
        }
    }

    while !queue.is_empty() {
        let connectors: Vec<usize> = queue.iter()
            .flat_map(|neuron_id| network.get_neuron(neuron_id).to_arr.clone())
            .collect();

        // Set of neurons that arent verified to be in the next layer, so temp
        let new_layer = connectors.into_iter()
            .map(|connector_id|{
                let neuron = network.connectors[connector_id].to;

                network.neurons[
                    *network.neuron_map.get(&neuron).unwrap()
                ].calls += 1;

                neuron
            })
            .collect::<HashSet<usize>>()
            .into_iter()
            .filter(|neuron_id|{
                let neuron = network.get_neuron(neuron_id);
            
                neuron.calls == neuron.from_arr.len()
            })
            .collect::<Vec<usize>>();

        layers.push(queue.clone());
        queue = new_layer;
    }

    layers[1] = layers[1].iter()
    .filter(|neuron_id| !input_hash.contains(*neuron_id))
    .cloned()
    .collect::<Vec<usize>>();


    #[cfg(debug_assertions)]
    {
        let mut len: usize = 0;
        for layer in layers.iter() {
            len += layer.len();
        }

        if len != network.neurons.len() {
            println!("\nError");

            for connection in network.connectors.iter() {
                println!("From: {:?}, To: {:?}", connection.from, connection.to);
            }
            
            panic!("Layering failed at layering. Layers: {:?}", layers);
        }
    }

    layers
}