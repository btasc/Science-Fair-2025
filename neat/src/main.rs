use neatcore::Core;

const POPULATION: usize = 150;

fn main() {
    let start = std::time::Instant::now();

    let mut core = Core::new();

    core.init_table(
        (vec![1], vec![2]),
        vec![],
    );

    core.init_genome(
        POPULATION,
        (vec![], vec![], vec![])
    );    

    core.mutate(1);
    println!("{:?}", core.gen_arr[1]);
}