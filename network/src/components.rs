pub struct Connector {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
}

pub struct Neuron {
    pub id: usize,
    pub from_arr: Vec<usize>,
    pub to_arr: Vec<usize>,
    pub value: f64,
    /*
    Calls is just for layers
    its incremented when the neuron is mentioned in the from connection of a connector
    it can then be check against from_arr.len
     */
    pub calls: usize,
}

pub type GenomeType = (Vec<usize>, Vec<f64>, Vec<bool>);