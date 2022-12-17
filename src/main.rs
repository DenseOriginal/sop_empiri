use rand::{thread_rng, Rng, rngs::ThreadRng};
use std::time::Instant;
use std::thread::{self, JoinHandle};

fn main() {
    test_algoritm(sorts::insertion_sort);
}

fn test_algoritm(sort_fn: fn(&mut [u64])) {
    const AVERAGE_OVER_N_RUNS: i8 = 10;
    const SIZE: u64 = 1000;
    const SAMPLE_POINTS: u8 = 100;
    
    for i in 1..=SAMPLE_POINTS {
        let length: u64 = SIZE * i as u64;

        let handlers: Vec<JoinHandle<u128>> = (0..AVERAGE_OVER_N_RUNS).map(|_| {
            return thread::spawn(move || {
                let mut rng: ThreadRng = thread_rng();
                let mut vals: Vec<u64> = (0..length).map(|_| rng.gen_range(0..1000)).collect();
                let start: Instant = Instant::now();
                sort_fn(&mut vals);
                return start.elapsed().as_micros();
            });
        }).collect();

        let duration: u128 = handlers
            .into_iter()
            .map(|handler| { handler.join().unwrap() })
            .sum();
    
        println!("Average time for sorting {} values {:?} micro seconds", length, duration / AVERAGE_OVER_N_RUNS as u128);
    }
}
