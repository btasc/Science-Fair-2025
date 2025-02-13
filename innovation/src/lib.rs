use std::collections::HashMap;

pub type RawInnovation = (usize, usize, i32);

pub struct Innovation {
    pub from: usize,
    pub to: usize,
    pub id: usize,
    pub neuron: i32, // = -1 if its a connection
}

pub struct InnovationTable {
    pub innovations: Vec<Innovation>,
    pub innovation_map: HashMap<RawInnovation, usize>, // (from, to, neuron) -> id. You can then get the innovation from the innovations vec
    pub neuron_levels: (Vec<usize>, Vec<usize>),
}

impl InnovationTable {
    pub fn new() -> InnovationTable {
        InnovationTable {
            innovations: Vec::new(),
            innovation_map: HashMap::new(),
            neuron_levels: (Vec::new(), Vec::new()),
        }
    }

    pub fn add_innovation(&mut self, innovation: RawInnovation) {

        #[cfg(debug_assertions)]
        {
            if innovation.2 < -1 {
                panic!("Neuron id must be -1 or greater");
            }

            match self.get_innovation(innovation) {
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
            innovation,
            self.innovations.len() - 1
        );
    }

    pub fn get_innovation(&self, innovation: RawInnovation) -> Option<&usize> {
        match self.innovation_map.get(&innovation) {
            Some(id) => Some(id),
            None => None,
        }
    }

    pub fn set_levels(&mut self, input_level: Vec<usize>, output_level: Vec<usize>) {
        self.neuron_levels = (input_level, output_level);
    }
}