pub struct Connector {
    pub from: i32,
    pub to: i32,
    pub weight: i32,
    pub innovation_id: i32,
}

pub enum NeuronType {
    Input,
    Output,
    Hidden,
}

pub struct Neuron {
    pub id: i32,
    pub kind: NeuronType,
    pub from_arr: Vec<i32>,
    pub to_arr: Vec<i32>,
}