extern crate network;
use innovation::*;
use network::*;

#[test]
fn layering() {
    // ! Network 1 - Simple
    let genome1 = Genome(vec![0, 1, 2], vec![0.1, 0.2, 0.3], vec![true, false, true]);
    let mut table1 = InnovationTable::new();

    table1.add_innovation((1, 2, Type::Connector));
    table1.add_innovation((2, 3, Type::Connector));
    table1.add_innovation((0, 3, Type::Connector));

    let network = NeuralNetwork::init(&genome1, &table1);

    assert_eq!(network.layers, vec![vec![0], vec![1], vec![2], vec![3]]);

    // ! Network 2 - Advanced
    let genome2 = Genome(vec![0, 1, 2], vec![0.1, 0.2, 0.3], vec![true, false, true]);
    let mut table2 = InnovationTable::new();
    table2.set_levels(vec![1], vec![2]);

    table2.add_innovation((1, 3, Type::Connector));
    table2.add_innovation((3, 2, Type::Connector));
    table2.add_innovation((4, 3, Type::Connector));
    table2.add_innovation((5, 2, Type::Connector));
    table2.add_innovation((0, 5, Type::Connector));
    table2.add_innovation((0, 3, Type::Connector));

    let network = NeuralNetwork::init(&genome2, &table2);

    assert_eq!(network.layers, vec![vec![0], vec![1, 5], vec![2, 4], vec![3]]);

}