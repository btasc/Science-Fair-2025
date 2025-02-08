use innovation::InnovationTable;
use network::NeuralNetwork;

fn main() {
    let genome = (vec![0], vec![0.9], vec![true]);
    let mut innovation_table = InnovationTable::new();

    innovation_table.add_innovation(1, 2, -1);
    innovation_table.set_levels(vec![1], vec![2]);

    NeuralNetwork::init(genome, &innovation_table);
}