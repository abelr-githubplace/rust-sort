use std::cmp::Ordering;

pub fn insertion_sort<T: Ord>(v: &mut [T]) {
    for i in 1..=v.len() {
        let mut j = i - 1;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn insertion_sort_by<T, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..=v.len() {
        let mut j = i - 1;
        while j > 0 && f(&v[j - 1], &v[j]) == Ordering::Greater {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{in_place_test, insertion_sort, insertion_sort_by};

    in_place_test!(insertion_sort);
    in_place_test!(insertion_sort_by, |a, b| a.cmp(b));
}
