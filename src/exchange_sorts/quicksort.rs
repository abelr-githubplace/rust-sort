use std::cmp::Ordering;

pub fn _exchange<T: Ord>(vec: &mut [T], pivot: &T) -> (usize, usize) {
    let length = vec.len();
    let mut left_idx = 0;
    let mut right_idx = length - 1;
    loop {
        while left_idx < length && vec[left_idx] <= *pivot {
            left_idx += 1;
        }
        while vec[right_idx] > *pivot {
            right_idx -= 1;
        }
        if left_idx >= right_idx {
            break;
        }
        vec.swap(left_idx, right_idx);
        left_idx += 1;
        right_idx -= 1;
    }
    (left_idx, right_idx + 1)
}

pub fn _move_pivots<T: Ord>(v: &mut [T], pivot: &T, mut pivot_idx: usize) -> (usize, usize) {
    let mut left_idx = 0;
    pivot_idx -= 1;

    loop {
        while pivot_idx > 0 && v[pivot_idx] == *pivot {
            pivot_idx -= 1;
        }

        while v[left_idx] != *pivot {
            left_idx += 1;
        }

        if left_idx >= pivot_idx {
            break;
        }

        v.swap(left_idx, pivot_idx);
        left_idx += 1;
        pivot_idx -= 1;
    }

    (left_idx, pivot_idx)
}

pub fn _pivot<T: Ord + Clone>(v: &mut [T]) -> T {
    let len = v.len();
    let middle = len / 2;

    if v[0] > v[middle] {
        v.swap(0, middle);
    }

    if v[middle] > v[len - 1] {
        v.swap(middle, len - 1);
    }

    if v[0] > v[middle] {
        v.swap(0, middle);
    }

    v[middle].clone()
}

pub fn quicksort<T: Ord + Clone>(v: &mut [T]) {
    let len = v.len();

    if len < 2 {
        return;
    }

    let pivot = _pivot(v);

    if len < 4 {
        return;
    }

    let (pivot_idx, right_idx) = _exchange(v, &pivot);
    let (left_idx, _) = _move_pivots(v, &pivot, pivot_idx);
    quicksort(&mut v[right_idx..]);
    quicksort(&mut v[..left_idx]);
}

#[allow(unused_variables)]
pub fn quicksort_by<T: Clone, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    unimplemented!("Missing implementation for `quicksort_by()`")
}

#[cfg(test)]
mod tests {
    use crate::{in_place_test, quicksort};
    //use crate::quicksort_by;

    in_place_test!(quicksort);
    //in_place_test!(quicksort_by, |a, b| a.cmp(b));
}
