use innovation::NeuronType;

pub struct Connector {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
}

pub struct Neuron {
    pub id: usize,
    pub kind: NeuronType,
    pub from_arr: Vec<usize>,
    pub to_arr: Vec<usize>,
    pub value: f64,
}

pub type GenomeType = (Vec<usize>, Vec<f64>, Vec<bool>);