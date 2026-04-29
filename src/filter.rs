pub struct MovingAverage {
    window: Vec<f64>,
    size: usize,
}

impl MovingAverage {
    pub fn new(size: usize) -> Self {
        Self {
            window: Vec::new(),
            size,
        }
    }

    pub fn apply(&mut self, value: f64) -> f64 {
        self.window.push(value);

        if self.window.len() > self.size{
            self.window.remove(0); // remove oldest value
        }

        let sum: f64 = self.window.iter().sum();
        sum / self.window.len() as f64
    }
}