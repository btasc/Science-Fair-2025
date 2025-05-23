extern crate network;
use innovation::InnovationTable;
use network::*;

#[test]
fn layering() {
    let genome = Genome(vec![1, 2, 3], vec![0.1, 0.2, 0.3], vec![true, false, true]);
    let table = InnovationTable::new();
}