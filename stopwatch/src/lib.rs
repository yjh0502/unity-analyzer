use std::time::Instant;

pub struct Stopwatch {
    start: Instant,
}

impl Stopwatch {
    pub fn start_new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    pub fn restart(&mut self) {
        self.start = Instant::now();
    }
}
