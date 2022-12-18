use csv::Writer;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;
mod algs;

fn main() {
    let average_over_n_runs: usize = 5;
    const SIZE: u64 = 1000;
    const SAMPLES: u16 = 50;

    let steps = (1..=SAMPLES).map(|i| SIZE * i as u64).collect::<Vec<u64>>();

    println!("\nMerge sort, multi thread");
    let merge_data = test_algoritm_multi(algs::merge_sort, average_over_n_runs, steps.clone());

    println!("\nBubble sort, multi thread");
    let bubble_data = test_algoritm_multi(algs::bubble_sort, average_over_n_runs, steps.clone());

    let csv_name = format!("merge, bubble step={} samples={}.csv", SIZE, SAMPLES);
    
    let mut wtr = Writer::from_path(csv_name).unwrap();
    // wtr.write_record(&["N", "Insertion sort", "Merge sort", "Bubble sort", "Heap sort"])
    wtr.write_record(&["N", "Merge sort", "Bubble sort"])
        .unwrap();
    for (i, n) in steps.into_iter().enumerate() {
        wtr.write_record(&[
            n.to_string(),
            // insertion_data[i].to_string(),
            merge_data[i].to_string(),
            bubble_data[i].to_string(),
            // heap_data[i].to_string(),
        ])
        .unwrap();
    }

    wtr.flush().unwrap();
}

fn test_algoritm_multi(
    sort_fn: fn(&mut [u64]) -> usize,
    average_over_n_runs: usize,
    steps: Vec<u64>,
) -> Vec<usize> {
    return steps.par_iter().map(|n| -> usize {
        let mut rng: ThreadRng = thread_rng();

        let mut comparisons_sum = 0;
        for _ in 1..average_over_n_runs {
            let mut vals: Vec<u64> = (0..*n).map(|_| rng.gen_range(0..1000)).collect();

            let comparisons = sort_fn(&mut vals);

            comparisons_sum += comparisons;
        }

        println!(
            "{}: {:?} Comparisons",
            n,
            comparisons_sum / average_over_n_runs
        );

        return comparisons_sum / average_over_n_runs;
    }).collect();
}
