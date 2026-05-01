mod signal;
mod filter;
mod peak;
mod bpm;

use std::{thread, time::Duration};
use signal::Signal;
use filter::MovingAverage;
use peak::PeakDetector;
use bpm::BpmCalculator;

fn main() {
    let mut signal = Signal::new();
    let mut filter = MovingAverage::new(5); // window size = 5
    let mut detector = PeakDetector::new();
    let mut bpm = BpmCalculator::new();

    loop {
        let raw = signal.next();
        let smooth = filter.apply(raw);

        if let Some(beat_time) = detector.detect(smooth) {
            println!("💓 Beat detected!");
            if let Some(value) = bpm.record_beat(beat_time) {
                println!("Current BPM: {:.2}", value);
            }
        }

        thread::sleep(Duration::from_millis(50));

    }
}