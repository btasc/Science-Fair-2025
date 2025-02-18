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

    pub fn init_genome(&mut self, population: usize, genome: GenomeType) {
        #[cfg(debug_assertions)]
        {
            if genome.0.len() != genome.1.len() || genome.0.len() != genome.2.len() || genome.1.len() != genome.2.len() {
                panic!("Genome length mismatch at neatcore");
            }
        }

        for _ in 0..population {
            self.gen_arr.push(genome.clone());
        }
    }

    pub fn run(&self, index: usize, inputs: Vec<f64>) -> Vec<f64> {
        #[cfg(debug_assertions)]
        {
            if inputs.len() != self.table.neuron_levels.0.len() {
                panic!("Input length does not match input layer length at neatcore");
            }
        }

        let genome = self.gen_arr[index].clone();

        let mut network = NeuralNetwork::init(genome, &self.table);
        network.run(inputs)
    }

    pub fn compare(&self, index1: usize, index2: usize) {
        let genome1 = &self.gen_arr[index1];
        let genome2 = &self.gen_arr[index2];

        let len1 = genome1.0.len();
        let len2 = genome2.0.len();

        let minLength = std::cmp::min(len1, len2);

        let mut marked1: Vec<bool> = vec![false; len1];
        let mut marked2: Vec<bool> = vec![false; len2];

        let mut matching: Vec<usize> = Vec::new();
        let mut disjoint: usize = 0;
        let mut excess: usize = 0;

        // Get all matching
        for i in 0..minLength {
            if genome1.0[i] == genome2.0[i] {
                matching.push(i);

                marked1[i] = true;
                marked2[i] = true;
            }
        }

        for i in 0..minLength {
            if marked1[i] {
                continue;
            }

            for j in 0..minLength {
                if marked2[i] {
                    continue;
                }

                if genome1.0[i] == genome2.0[j] {
                    disjoint += 1;

                    marked1[i] = true;
                    marked2[j] = true;
                }
            }
        }

        disjoint = marked1.iter().filter(|&&x| !x).count();
        disjoint += marked2.iter().filter(|&&x| !x).count();

        let average_matching_weight = 0;


    }
}