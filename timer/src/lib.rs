use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Option<Instant>,
    elapsed_time: Option<Duration>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: None,
            elapsed_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.elapsed_time = None;
    }

    pub fn stop(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed_time = Some(start.elapsed());
            self.start_time = None;
        }
    }

    pub fn log(&self) {
        match self.elapsed_time {
            Some(duration) => println!("Elapsed time: {:?}", duration),
            None => println!("Timer has not been stopped yet or has not been started."),
        }
    }
}