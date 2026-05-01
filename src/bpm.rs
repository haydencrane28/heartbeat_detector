use std::time::Instant;

pub struct BpmCalculator {
    last_beat: Option<Instant>,
}

impl BpmCalculator {
    pub fn new() -> Self {
        Self { last_beat: None }
    }

    pub fn record_beat(&mut self, beat_time: Instant) -> Option<f64> {
        match self.last_beat{
            None => {
                self.last_beat = Some(beat_time);
                None
            }
            Some(previous) => {
                let elapsed = beat_time.duration_since(previous).as_secs_f64();
                self.last_beat = Some(beat_time);
                if elapsed > 0.0 {
                    Some(60.0 / elapsed) // Convert to BPM
                } else {
                    None
                }
            }
        }
    }
}