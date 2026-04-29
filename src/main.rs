mod signal;
mod filter;
mod peak;

use std::{thread, time::Duration};
use signal::Signal;
use filter::MovingAverage;
use peak::PeakDetector;

fn main() {
    let mut signal = Signal::new();
    let mut filter = MovingAverage::new(5); // window size = 5
    let mut detector = PeakDetector::new();

    loop {
        let raw = signal.next();
        let smooth = filter.apply(raw);

        //println!("raw: {:.2}, smooth: {:.2}", raw, smooth);
        if let Some(_) = detector.detect(smooth) {
            println!("💓 Beat detected!");
        }

        thread::sleep(Duration::from_millis(50));

    }
}