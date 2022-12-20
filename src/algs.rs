#[allow(dead_code)]
pub fn merge_sort(x: &mut [u64]) -> usize {
    let n = x.len();
    let m = n / 2;
    let mut comparisons = 0;

    if n <= 1 {
        return 0;
    }

    comparisons += merge_sort(&mut x[0..m]);
    comparisons += merge_sort(&mut x[m..n]);

    let mut y: Vec<u64> = x.to_vec();

    comparisons += merge(&x[0..m], &x[m..n], &mut y[..]);

    x.copy_from_slice(&y);

    comparisons
}

#[allow(dead_code)]
fn merge(x1: &[u64], x2: &[u64], y: &mut [u64]) -> usize {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut comparisons = 0;

    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
        comparisons += 1;
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }

    comparisons
}

#[allow(dead_code)]
pub fn bubble_sort(arr: &mut [u64]) -> usize {
    let mut comparisons = 0;

    let mut n = arr.len();
    while n > 1 {
        let mut new_n = 0;
        for i in 1..n {
            comparisons += 1;
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
    }

    comparisons
}

#[allow(dead_code)]
pub fn quick_sort(list: &mut [u64]) -> usize {
    let mut comparisons = 0;

    if list.len() <= 1 {
        return 0;
    }

    let pivot = list[0];
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in 1..list.len() {
        comparisons += 1;
        if list[i] < pivot {
            left.push(list[i]);
        } else {
            right.push(list[i]);
        }
    }

    comparisons += quick_sort(&mut left);
    comparisons += quick_sort(&mut right);

    comparisons
}

#[allow(dead_code)]
const MIN_RUN: usize = 32;
#[allow(dead_code)]
pub fn timsort(arr: &mut [u64]) -> usize {
    let mut comparisons = 0;

    // Sort small arrays with insertion sort
    fn insertion_sort(arr: &mut [u64], comparisons: &mut usize) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                j -= 1;
                *comparisons += 1;
            }
        }
    }

    // Merge larger subarrays with merge_sort
    fn merge_sort(
        arr: &mut [u64],
        aux: &mut [u64],
        start: usize,
        end: usize,
        comparisons: &mut usize,
    ) {
        if end - start > 1 {
            let mid = start + (end - start) / 2;
            merge_sort(arr, aux, start, mid, comparisons);
            merge_sort(arr, aux, mid, end, comparisons);

            let mut i = start;
            let mut j = mid;
            let mut k = start;
            while k < end {
                if i < mid && (j >= end || arr[i] < arr[j]) {
                    aux[k] = arr[i];
                    i += 1;
                } else {
                    aux[k] = arr[j];
                    j += 1;
                }
                *comparisons += 1;
                k += 1;
            }

            for k in start..end {
                arr[k] = aux[k];
            }
        }
    }

    // Find the length of the run (consecutive ascending elements) at the start of the array
    let mut run_len = 1;
    while run_len < arr.len() {
        if arr[run_len] < arr[run_len - 1] {
            break;
        }
        run_len += 1;
    }

    // Reverse the run if it is descending
    if run_len < arr.len() && arr[run_len] < arr[run_len - 1] {
        for i in 0..run_len / 2 {
            arr.swap(i, run_len - 1 - i);
        }
    }

    // Sort the rest of the array
    let mut i = run_len;
    while i < arr.len() {
        // Find the length of the next run
        let mut run_len = 1;
        while i + run_len < arr.len() {
            if arr[i + run_len - 1] > arr[i + run_len] {
                comparisons += 1;
                break;
            }
            run_len += 1;
        }

        // Reverse the run if it is descending
        if i + run_len < arr.len() && arr[i + run_len - 1] > arr[i + run_len] {
            for j in 0..run_len / 2 {
                comparisons += 1;
                arr.swap(i + j, i + run_len - 1 - j);
            }
        }

        // Check if the current run is too small to merge
        if run_len < MIN_RUN {
            insertion_sort(&mut arr[i..i + run_len], &mut comparisons);
            i += run_len;
            continue;
        }

        // Check if the current run is the last in the array
        if i + run_len >= arr.len() {
            run_len = arr.len() - i;
        } else {
            // Find the length of the next run
            let mut next_run_len = 1;
            while i + run_len + next_run_len < arr.len() {
                comparisons += 1;
                if arr[i + run_len + next_run_len - 1] > arr[i + run_len + next_run_len] {
                    break;
                }
                next_run_len += 1;
            }

            // Check if the current and next runs are already in ascending order
            if arr[i + run_len - 1] <= arr[i + run_len] {
                run_len += next_run_len;
            }
        }

        // Sort and merge the current run
        let mut aux = Vec::with_capacity(run_len);
        aux.extend_from_slice(&arr[i..i + run_len]);
        merge_sort(
            &mut aux,
            &mut arr[i..i + run_len],
            0,
            run_len,
            &mut comparisons,
        );
        i += run_len;
    }

    comparisons
}

#[allow(dead_code)]
pub fn insertion_sort(arr: &mut [u64]) -> usize {
    let mut comparisons = 0;

    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            comparisons += 1;
            arr.swap(j, j - 1);
            j -= 1;
        }
    }

    comparisons
}
