use std::{collections::HashMap, thread::panicking};

pub type RawInnovation = (usize, usize, bool);

pub struct Innovation {
    pub from: usize,
    pub to: usize,
    pub id: usize,
    pub neuron: bool, // = false if its a connection
}

pub struct InnovationTable {
    pub innovations: Vec<Innovation>,
    pub innovation_map: HashMap<(usize, usize, bool), usize>, // (from, to, neuron) -> id. You can then get the innovation from the innovations vec
    pub neuron_levels: (Vec<usize>, Vec<usize>),
    neuron_counter: usize,
}

impl InnovationTable {
    pub fn new() -> InnovationTable {
        InnovationTable {
            innovations: Vec::new(),
            innovation_map: HashMap::new(),
            neuron_levels: (Vec::new(), Vec::new()),
            neuron_counter: 0,
        }
    }

    pub fn add_innovation(&mut self, innovation: RawInnovation) {

        #[cfg(debug_assertions)]
        {
            match self.get_innovation((innovation.0, innovation.1, innovation.2)) {
                Some(_) => panic!("Innovation already exists"),
                None => (),
            }
        }

        self.innovations.push(
            Innovation {
                from: innovation.0,
                to: innovation.1,
                id: self.innovations.len(),
                neuron: innovation.2,
            }
        );

        self.innovation_map.insert(
            (innovation.0, innovation.1, innovation.2),
            self.innovations.len() - 1
        );
    }

    pub fn get_innovation(&self, innovation: (usize, usize, bool)) -> Option<&usize> {
        match self.innovation_map.get(&(innovation.0, innovation.1, innovation.2)) {
            Some(index) => Some(index),
            None => None,
        }
    }

    pub fn set_levels(&mut self, input_level: Vec<usize>, output_level: Vec<usize>) {
        #[cfg(debug_assertions)]
        {
            for i in 0..input_level.len() {
                if i+1 != input_level[i] {
                    panic!("inputs not sequential at innovation");
                }
            }

            for i in 0..output_level.len() {
                if i+input_level.len()+1 != output_level[i] {
                    panic!("outputs not sequential at innovation");
                }
            }  
        }

        self.neuron_counter = input_level.len() + output_level.len();
        self.neuron_levels = (input_level, output_level);
    }
}