use crate::{heapsort, insertion_sort, quicksort};
use std::cmp::Ordering;

fn _introsort<T: Ord + Clone>(v: &mut [T], depth: usize) {
    let l = v.len();

    if l <= 16 {
        insertion_sort(v);
        return;
    }

    if depth == 0 {
        heapsort(v);
        return;
    }

    let pivot = quicksort::_pivot(v);
    let (pivot_idx, right_idx) = quicksort::_exchange(v, &pivot);
    let (left_idx, _) = quicksort::_move_pivots(v, &pivot, pivot_idx);
    _introsort(&mut v[right_idx..], depth - 1);
    _introsort(&mut v[..left_idx], depth - 1);
}

pub fn introsort<T: Ord + Clone>(v: &mut [T]) {
    let max_depth = ((v.len() as f64).log2() * 2.) as usize;
    _introsort(v, max_depth)
}

#[allow(unused_variables)]
pub fn introsort_by<T: Clone, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    unimplemented!("Missing implementation for `introsort_by()`")
}

#[cfg(test)]
mod tests {
    use crate::{in_place_test, introsort};
    //use crate::introsort_by;

    in_place_test!(introsort);
    //in_place_test!(introsort_by, |a, b| a.cmp(b));
}
