use rand::{thread_rng, Rng, rngs::ThreadRng};
use std::time::Instant;

fn main() {
    test_algoritm(sorts::insertion_sort);
}

fn test_algoritm(sort_fn: fn(&mut [u64])) {
    let mut rng: ThreadRng = thread_rng();

    const AVERAGE_OVER_N_RUNS: i8 = 10;
    const SIZE: u64 = 1000;
    const SAMPLE_POINTS: u8 = 100;
    
    for i in 1..=SAMPLE_POINTS {
        let mut duration_sum: u128 = 0;
        let legth: u64 = SIZE * i as u64;

        for _ in 1..AVERAGE_OVER_N_RUNS {
            let mut vals: Vec<u64> = (0..legth).map(|_| rng.gen_range(0..1000)).collect();
    
            let start: Instant = Instant::now();
            sort_fn(&mut vals);
            let duration: u128 = start.elapsed().as_micros();
    
            duration_sum += duration;
        }
    
        println!("Average time for sorting {} values {:?} micro seconds", legth, duration_sum / AVERAGE_OVER_N_RUNS as u128);
    }
}
