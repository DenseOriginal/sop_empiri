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