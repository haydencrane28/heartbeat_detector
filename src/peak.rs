use std::time::Instant;

pub struct PeakDetector{
    prev: f64,
    curr: f64,
    initialized: bool,
}

impl PeakDetector {
    pub fn new() -> Self {
        Self {
            prev: 0.0,
            curr: 0.0,
            initialized: false,
        }
    }

    pub fn detect(&mut self, next: f64) -> Option<Instant> {
        if !self.initialized {
            self.curr = next;
            self.initialized = true;
            return None;
        }

        // check for peak
        if self.prev < self.curr && self.curr > next {
            let peak_time = Instant::now();

            // Shift values forward
            self.prev = self.curr;
            self.curr = next;

            return Some(peak_time);
        }

        //Shfit values forward
        self.prev = self.curr;
        self.curr = next;
        None
    }
}