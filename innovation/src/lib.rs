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
    pub neuron_levels: (Vec<i32>, Vec<i32>),
}

pub enum NeuronType {
    Input,
    Output,
    Hidden,
}

impl InnovationTable {
    pub fn new() -> InnovationTable {
        InnovationTable {
            innovations: Vec::new(),
            innovation_map: HashMap::new(),
            neuron_levels: (Vec::new(), Vec::new()),
        }
    }

    pub fn add_innovation(&mut self, from: i32, to: i32, neuron: i32) {

        #[cfg(debug_assertions)]
        {
            if neuron < -1 {
                panic!("Neuron id must be -1 or greater");
            }

            match self.get_innovation(from, to, neuron) {
                Some(_) => panic!("Innovation already exists"),
                None => (),
            }
        }

        self.innovations.push(
            Innovation {
                from,
                to,
                id: self.innovations.len() as i32,
                neuron,
            }
        );

        self.innovation_map.insert(
            (from, to, neuron),
            self.innovations.len() as i32
        );
    }

    pub fn get_innovation(&self, from: i32, to: i32, neuron: i32) -> Option<&i32> {
        match self.innovation_map.get(&(from, to, neuron)) {
            Some(id) => Some(id),
            None => None,
        }
    }

    pub fn set_levels(&mut self, input_level: Vec<i32>, output_level: Vec<i32>) {
        self.neuron_levels = (input_level, output_level);
    }

    pub fn get_levels(&self, neuron: i32) -> NeuronType {
        if self.neuron_levels.0.contains(&neuron) {
            NeuronType::Input
        } else if self.neuron_levels.1.contains(&neuron) {
            NeuronType::Output
        } else {
            NeuronType::Hidden
        }
    }
}