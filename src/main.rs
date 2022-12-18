use csv::Writer;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::thread::{self, JoinHandle};
use std::time::Instant;

fn main() {
    const AVERAGE_OVER_N_RUNS: i8 = 5;
    const SIZE: u64 = 500;
    const SAMPLES: u8 = 60;

    let steps = (1..=SAMPLES).map(|i| SIZE * i as u64).collect::<Vec<u64>>();

    println!("\nInsertion sort, multi thread");
    let insertion_data =
        test_algoritm_multi(sorts::insertion_sort, AVERAGE_OVER_N_RUNS, SIZE, SAMPLES);

    println!("\nMerge sort, multi thread");
    let merge_data = test_algoritm_multi(sorts::merge_sort, AVERAGE_OVER_N_RUNS, SIZE, SAMPLES);

    let mut wtr = Writer::from_path("sorting-data.csv").unwrap();
    wtr.write_record(&["N", "Insertion sort", "Merge sort"])
        .unwrap();
    for (i, n) in steps.iter().enumerate() {
        wtr.write_record(&[
            n.to_string(),
            insertion_data[i].to_string(),
            merge_data[i].to_string(),
        ])
        .unwrap();
    }

    wtr.flush().unwrap();
}

fn test_algoritm_multi(
    sort_fn: fn(&mut [u64]),
    average_over_n_runs: i8,
    size: u64,
    samples: u8,
) -> Vec<u128> {
    let mut results: Vec<u128> = Vec::new();

    for i in 1..=samples {
        let length: u64 = size * i as u64;

        let handlers: Vec<JoinHandle<u128>> = (0..average_over_n_runs)
            .map(|_| {
                return thread::spawn(move || {
                    let mut rng: ThreadRng = thread_rng();
                    let mut vals: Vec<u64> = (0..length).map(|_| rng.gen_range(0..1000)).collect();
                    let start: Instant = Instant::now();
                    sort_fn(&mut vals);
                    return start.elapsed().as_micros();
                });
            })
            .collect();

        let duration: u128 = handlers
            .into_iter()
            .map(|handler| handler.join().unwrap())
            .sum();

        results.push(duration / average_over_n_runs as u128);
        println!(
            "{}: {:?} microseconds",
            length,
            duration / average_over_n_runs as u128
        );
    }

    return results;
}

fn test_algoritm_single(
    sort_fn: fn(&mut [u64]),
    average_over_n_runs: i8,
    size: u64,
    samples: u8,
) -> Vec<u128> {
    let mut results: Vec<u128> = Vec::new();
    let mut rng: ThreadRng = thread_rng();

    for i in 1..=samples {
        let mut duration_sum: u128 = 0;
        let length: u64 = size * i as u64;

        for _ in 1..average_over_n_runs {
            let mut vals: Vec<u64> = (0..length).map(|_| rng.gen_range(0..1000)).collect();

            let start: Instant = Instant::now();
            sort_fn(&mut vals);
            let duration: u128 = start.elapsed().as_micros();

            duration_sum += duration;
        }

        results.push(duration_sum / average_over_n_runs as u128);
        println!(
            "{}: {:?} microseconds",
            length,
            duration_sum / average_over_n_runs as u128
        );
    }

    return results;
}
