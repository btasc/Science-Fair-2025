use innovation::InnovationTable;
use network::NeuralNetwork;

fn main() {
    let genome = (vec![0, 1, 2, 3, 4], vec![0.9, -0.1, 1.0, 1.0, 1.0], vec![true, true, true, true, true]);
    let mut innovation_table = InnovationTable::new();

    innovation_table.add_innovation(2, 3, -1);
    innovation_table.add_innovation(5, 3, -1);
    innovation_table.add_innovation(6, 3, -1);
    innovation_table.add_innovation(1, 6, -1);
    innovation_table.add_innovation(7, 4, -1);
    innovation_table.set_levels(vec![1, 2], vec![3, 4]);

    NeuralNetwork::init(genome, &innovation_table);

    println!("hi");
}