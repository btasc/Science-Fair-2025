extern crate network;
use innovation::*;
use network::*;

fn sort_layers(mut layers: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut sorted_layers: Vec<Vec<usize>> = Vec::new();

    for layer in &layers {
        let mut new_layer = layer.clone();
        new_layer.sort();

        sorted_layers.push(new_layer);
    }

    sorted_layers
}

#[test]
fn layering() {
    // ! Network 1 - Simple
    let genome1 = Genome(vec![0, 1, 2], vec![0.1, 0.2, 0.3], vec![true, true, true]);

    let table1 = InnovationTable::init(
        (vec![1], vec![2]),
        vec![
           (1, 2, Type::Connector),
           (0, 2, Type::Connector),
           (0, 3, Type::Connector),
        ]
    );

    let network = NeuralNetwork::init(&genome1, &table1);

    let network_sorted = sort_layers(network.layers);
    let actual_sorted = sort_layers(vec![vec![0, 1], vec![4], vec![3, 5], vec![2]]);

    assert_eq!(network_sorted, actual_sorted);

    // ! Network 2 - Advanced
    let genome = Genome(vec![0, 1, 2, 3, 4, 5], vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6], vec![true, true, true, true, true, true]);

    let table = InnovationTable::init(
        (vec![1], vec![2]),
        vec![
           (1, 3, Type::Connector),
           (3, 2, Type::Connector),
           (4, 3, Type::Connector),
           (5, 2, Type::Connector),
           (0, 5, Type::Connector),
           (0, 3, Type::Connector),
        ]
    );

    let network = NeuralNetwork::init(&genome, &table);
    
    let network_sorted = sort_layers(network.layers);
    let actual_sorted = sort_layers(vec![vec![0, 1], vec![4], vec![3, 5], vec![2]]);

    assert_eq!(network_sorted, actual_sorted);

    // ! Network 3 - Neuron Simple
    let genome = Genome(vec![0, 1, 2], vec![1.1, 1.1, 1.1], vec![true, true, true]);

    let table = InnovationTable::init(
        (vec![1], vec![2]),
        vec![
            (1, 2, Type::Neuron),
            (1, 3, Type::Connector),
            (3, 2, Type::Connector),
        ]
    );

    let network = NeuralNetwork::init(&genome, &table);

    let network_sorted = sort_layers(network.layers);
    let actual_sorted = sort_layers(vec![vec![0, 1], vec![3], vec![2]]);

    assert_eq!(network_sorted, actual_sorted);
}