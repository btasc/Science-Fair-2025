use neatcore::Core;

const POPULATION: usize = 150;

fn main() {
    let start = std::time::Instant::now();

    let mut core = Core::new();

    core.init_table(
        (vec![1], vec![2]),
        vec![
            (0, 2, -1)
        ],
    );

    core.init_genome(
        POPULATION,
        (vec![0], vec![1.0], vec![true])
    );

    for i in 0..POPULATION {
        core.compare(i, (i + 1) % POPULATION);
    }

    let time = start.elapsed();
    println!("Time elapsed: {:?}", time);
}