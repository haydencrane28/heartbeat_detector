use std::time::{Duration, Instant};

pub struct PeakDetector{
    prev: f64,
    curr: f64,
    initialized: bool,
    last_peak: Option<Instant>,
    min_interval: Duration,
}

impl PeakDetector {
    pub fn new() -> Self {
        Self {
            prev: 0.0,
            curr: 0.0,
            initialized: false,
            last_peak: None,
            min_interval: Duration::from_millis(300), // Minimum interval between peaks
        }
    }

    pub fn detect(&mut self, next: f64) -> Option<Instant> {
        if !self.initialized {
            self.curr = next;
            self.initialized = true;
            return None;
        }

        let now = Instant::now();

        let is_peak = self.prev < self.curr && self.curr > next;

        let allowed = match self.last_peak {
            None => true,
            Some(last) => now.duration_since(last) >= self.min_interval,
        };

        if is_peak && allowed {
            self.last_peak = Some(now);
            self.prev = self.curr;
            self.curr = next;

            return Some(now);
        }

        self.prev = self.curr;
        self.curr = next;

        None
    }
}