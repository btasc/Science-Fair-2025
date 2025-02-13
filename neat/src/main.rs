use neatcore::Core;

fn main() {
    let start = std::time::Instant::now();

    let core = Core::new();

    core.init_table(
        ()
    );

    let time = start.elapsed();
    println!("Time elapsed: {:?}", time);
}
