pub mod metrics;
pub use metrics::cpu::get_timestamp_counter;

pub struct Profiler {
    start: u64,
    end: u64,
}

impl Profiler {
    pub fn new() -> Self {
        Self { start: 0, end: 0 }
    }

    pub fn register_start(&mut self) {
        self.start = get_timestamp_counter();
    }

    pub fn register_end(&mut self) {
        self.end = get_timestamp_counter();
    }

    pub fn duration(&self) -> u64 {
        self.end - self.start
    }
}
