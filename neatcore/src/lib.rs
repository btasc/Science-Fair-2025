use network::NeuralNetwork;
use innovation::{InnovationTable, RawInnovation};

type GenomeType = (Vec<usize>, Vec<f64>, Vec<bool>);

pub struct Core {
    gen_arr: Vec<GenomeType>,
    table: InnovationTable,
}

impl Core {
    pub fn new() -> Core {
        Core {
            gen_arr: Vec::new(),
            table: InnovationTable::new(),
        }
    }

    pub fn init_table(&mut self, levels: (Vec<usize>, Vec<usize>), innovations: Vec<RawInnovation>) {
        #[cfg(debug_assertions)]
        {
            for output in &levels.1 {
                if levels.0.contains(output) {
                    panic!("Output neuron cannot be in input layer at neatcore");
                }
            }
        }

        self.table.set_levels(levels.0, levels.1);

        for innovation in innovations {
            self.table.add_innovation(innovation);
        }
    }
}