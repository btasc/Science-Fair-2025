use std::collections::HashMap;

pub struct Innovation {
    pub from: i32,
    pub to: i32,
    pub id: i32,
    pub neuron: i32, // = -1 if its a connection
}

pub struct InnovationTable {
    pub innovations: Vec<Innovation>,
    pub innovation_map: HashMap<(i32, i32, i32), i32>, // (from, to, neuron) -> id. You can then get the innovation from the innovations vec
}