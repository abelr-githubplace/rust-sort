use std::cmp::Ordering;

/// # BubleSort
///
/// |              | Time complexity  | Space Complexity | Stabilty | In-place |
/// |--------------|:----------------:|:----------------:|:--------:|:--------:|
/// | **Overall**  | *Quadratic time* | *Constant space* | Stable   | Yes      |
/// | Worst case   | `O(n^2)`         | `O(1)`           |
/// | Best case    | `O(n)`           | `O(1)`           |
/// | Average      | `O(n^2)`         | `O(1)`           |
///
pub fn bubble_sort<T: Ord>(v: &mut [T]) {
    let len = v.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            return;
        }
    }
}

/// # BubleSortCmp
///
/// |              | Time complexity  | Space Complexity | Stabilty | In-place |
/// |--------------|:----------------:|:----------------:|:--------:|:--------:|
/// | **Overall**  | *Quadratic time* | *Constant space* | Stable   | Yes      |
/// | Worst case   | `O(n^2)`         | `O(1)`           |
/// | Best case    | `O(n)`           | `O(1)`           |
/// | Average      | `O(n^2)`         | `O(1)`           |
///
pub fn bubble_sort_by<T, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = v.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - i - 1 {
            if f(&v[j], &v[j + 1]) == Ordering::Greater {
                v.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bubble_sort, bubble_sort_by, in_place_test};

    in_place_test!(bubble_sort);
    in_place_test!(bubble_sort_by, |a, b| a.cmp(b));
}
