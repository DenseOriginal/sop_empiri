use csv::Writer;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;
mod algs;

struct Algortihm {
    name: String,
    alg: fn(&mut [u64]) -> usize,
}

struct Results {
    name: String,
    data: Vec<usize>,
}

fn main() {
    let average_over_n_runs: usize = 5;
    const SIZE: usize = 10000;
    const SAMPLES: usize = 20;

    let steps = (1..=SAMPLES).map(|i| SIZE * i).collect::<Vec<usize>>();
    let steps_len = steps.len();

    let algs_to_test: Vec<Algortihm> = vec![
        Algortihm { name: String::from("Merge sort"), alg: algs::merge_sort },
        Algortihm { name: String::from("Quick sort"), alg: algs::quick_sort },
        Algortihm { name: String::from("Bubble sort"), alg: algs::bubble_sort },
        Algortihm { name: String::from("Insertion sort"), alg: algs::insertion_sort },
    ];

    let mut results: Vec<Results> = algs_to_test
        .into_iter()
        .map(|alg| -> Results {
            println!("\n{}", alg.name);
            let data = test_algoritm_multi(alg.alg, average_over_n_runs, steps.clone());
            return Results { name: alg.name, data: data };
        }).collect();

    results.reverse();
    results.push(Results { name: String::from("N"), data: steps });
    results.reverse();

    let csv_name = format!("merge vs hybrid step={} samples={}.csv", SIZE, SAMPLES);
    
    let mut wtr = Writer::from_path(csv_name).unwrap();
    // wtr.write_record(&["N", "Insertion sort", "Merge sort", "Bubble sort", "Heap sort"])
    wtr.write_record(results.iter().map(|data| data.name.clone()))
        .unwrap();
    for i in 0..steps_len {
        wtr.write_record(results.iter().map(|data| data.data[i].to_string()))
        .unwrap();
    }

    wtr.flush().unwrap();
}

fn test_algoritm_multi(
    sort_fn: fn(&mut [u64]) -> usize,
    average_over_n_runs: usize,
    steps: Vec<usize>,
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
