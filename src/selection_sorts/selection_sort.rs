use std::cmp::Ordering;

/// # SelectionSort
///
/// |              | Time complexity     | Space Complexity | Stabilty | In-place |
/// |--------------|:-------------------:|:----------------:|:--------:|:--------:|
/// | **Overall**  | *Quadratic time*    | *Constant space* | Stable   | Yes      |
/// | Worst case   | `O(n^2)`            | `O(1)`           |
/// | Best case    | `O(n)`              | `O(1)`           |
/// | Average      | `O(n^2)`            | `O(1)`           |
///
pub fn selection_sort<T: Ord>(v: &mut [T]) {
    let l = v.len();
    for i in 0..(l - 1) {
        let mut idx = i;
        for j in (i + 1)..l {
            if v[idx] > v[j] {
                idx = j;
            }
        }
        if i != idx {
            v.swap(i, idx);
        }
    }
}

#[allow(unused_variables)]
pub fn selection_sort_by<T, F>(v: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    unimplemented!("Missing implementation for `selection_sort_by()`")
}

#[cfg(test)]
mod tests {
    use crate::{in_place_test, selection_sort};
    //use crate::selection_sort_by;

    in_place_test!(selection_sort);
    //in_place_test!(selection_sort_by, |a, b| a.cmp(b));
}
