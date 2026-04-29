pub struct Signal {
    t: f64,
}

impl Signal {
    pub fn new() -> Self {
        Self { t: 0.0 }
    }

    pub fn next(&mut self) -> f64 {
        self.t += 0.1;

        // Fake heartbeat data signal
        let base = (self.t.sin() * 100.0) + 500.0;

        // Add slight noise
        let noise = rand::random::<f64>() * 10.0;

        base + noise
    }
}