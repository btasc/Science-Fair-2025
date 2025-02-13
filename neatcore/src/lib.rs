use network::NeuralNetwork;
use innovation::InnovationTable;


pub fn time_start() -> std::time::Instant {
    std::time::Instant::now()
}

pub fn time_stop(start: std::time::Instant) -> std::time::Duration {
    start.elapsed()
}