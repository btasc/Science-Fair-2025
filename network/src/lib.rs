pub struct Genome (Vec<usize>, Vec<f64>, Vec<bool>);

impl Genome {
    pub fn new (genes: Vec<usize>, weights: Vec<f64>, status: Vec<bool>) -> Self {
        Genome (genes, weights, status)
    }
}
