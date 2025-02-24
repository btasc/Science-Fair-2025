use neatcore::Core;

const POPULATION: usize = 150;

fn main() {    
    let mut core = Core::init(
        150, // # of genomes
        None, // None = (Vec::new(), Vec::new(), Vec::new())
        None, // None = No starting innovations
        (vec![1], vec![2]) // [input ids], [output ids]. Cant be 0 as 0 is bias and is always there
    ); 
    
    core.gen_arr[0] = (vec![1, 4, 2, 5, 3], vec![1.0, 1.0, 1.0, 1.0, 1.0], vec![true, true, true, true, true]);
    core.gen_arr[1] = (vec![0, 2, 3, 5], vec![-1.0, -1.0, -1.0, -1.0], vec![true, true, true, true]);

    println!("{:?}", core.crossover(0, 1));
}