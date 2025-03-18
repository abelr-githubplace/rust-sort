use std::cmp::Ordering;

#[inline]
fn heapify<T: Ord>(v: &mut [T], n: usize, mut i: usize) {
    let mut parent_idx = i;
    loop {
        let left_child_idx = 2 * i + 1;
        let right_child_idx = 2 * i + 2;
        if left_child_idx < n && v[left_child_idx] > v[parent_idx] {
            parent_idx = left_child_idx;
        }
        if right_child_idx < n && v[right_child_idx] > v[parent_idx] {
            parent_idx = right_child_idx;
        }
        if i != parent_idx {
            v.swap(i, parent_idx);
            i = parent_idx;
        } else {
            break;
        }
    }
}

#[inline]
fn build_heap<T: Ord>(v: &mut [T], n: usize) {
    for i in (0..n / 2).rev() {
        heapify(v, n, i);
    }
}

/// # SelectionSort
///
/// |              | Time complexity     | Space Complexity | Stabilty | In-place |
/// |--------------|:-------------------:|:----------------:|:--------:|:--------:|
/// | **Overall**  | *Quasi-linear time* | *Linear space*   | Stable   | Yes      |
/// | Worst case   | `O(n^2)`            | `O(1)`           |
/// | Best case    | `O(n)`              | `O(1)`           |
/// | Average      | `O(n^2)`            | `O(1)`           |
///
pub fn heapsort<T: Ord>(v: &mut [T]) {
    let l = v.len();

    build_heap(v, l);

    for i in 1..l {
        v.swap(0, l - i);
        heapify(&mut v[0..l - i], l - i, 0);
    }
}

#[allow(unused_variables)]
pub fn heapsort_by<T, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    unimplemented!("Missing implementation for `heapsortsort_by()`")
}

#[cfg(test)]
mod tests {
    use crate::{heapsort, in_place_test};
    //use crate::heapsort_by;

    in_place_test!(heapsort);
    //in_place_test!(heapsort_by, |a, b| a.cmp(b));
}
