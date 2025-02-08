use innovation::NeuronType;

pub struct Connector {
    pub from: i32,
    pub to: i32,
    pub weight: f64,
}

pub struct Neuron {
    pub id: i32,
    pub kind: NeuronType,
    pub from_arr: Vec<i32>,
    pub to_arr: Vec<i32>,
    pub value: f64,
}

pub type GenomeType = (Vec<usize>, Vec<f64>, Vec<bool>);