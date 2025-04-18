// Learning RUST: stopwatch#1
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    println!("⏱️ Stopwatch started...");

    let start = Instant::now();

    
    for i in 1..=10 {
        sleep(Duration::from_secs(1));
        println!("Elapsed time: {} seconds", start.elapsed().as_secs());
    }

    println!("⏹️ Stopwatch ended after 10 seconds.");
}
