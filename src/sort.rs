use crate::{
    bubble_sort, bubble_sort_by, heapsort, heapsort_by, insertion_sort, insertion_sort_by,
    introsort, introsort_by, merge_sort, merge_sort_by, quicksort, quicksort_by, selection_sort,
    selection_sort_by,
};
use std::cmp::Ordering;

pub enum Algorithm {
    BubbleSort,
    HeapSort,
    IntroSort,
    InsertionSort,
    SelectionSort,
    QuickSort,
    MergeSort,
}

pub enum CmpAlgorithm {
    BubbleSortCmp,
    HeapSortCmp,
    IntroSortCmp,
    InsertionSortCmp,
    SelectionSortCmp,
    QuickSortCmp,
    MergeSortCmp,
}

pub trait Sort {
    type Item;

    fn sort(&mut self, algo: Algorithm) -> &mut Self
    where
        Self::Item: Ord;

    fn sort_by<F>(&mut self, algo: CmpAlgorithm, compare: F) -> &mut Self
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering;
}

impl<T: Ord + Clone> Sort for Vec<T> {
    type Item = T;

    fn sort(&mut self, algo: Algorithm) -> &mut Self
    where
        Self::Item: Ord,
    {
        match algo {
            Algorithm::BubbleSort => {
                bubble_sort(self);
            }
            Algorithm::HeapSort => {
                heapsort(self);
            }
            Algorithm::InsertionSort => {
                insertion_sort(self);
            }
            Algorithm::IntroSort => {
                introsort(self);
            }
            Algorithm::QuickSort => {
                quicksort(self);
            }
            Algorithm::SelectionSort => {
                selection_sort(self);
            }
            Algorithm::MergeSort => {
                *self = merge_sort(self); // FIXME: Make `merge_sort()` in-place
            }
        }
        self
    }

    fn sort_by<F>(&mut self, algo: CmpAlgorithm, f: F) -> &mut Self
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering,
    {
        match algo {
            CmpAlgorithm::BubbleSortCmp => {
                bubble_sort_by(self, f);
            }
            CmpAlgorithm::HeapSortCmp => {
                heapsort_by(self, f);
            }
            CmpAlgorithm::IntroSortCmp => {
                introsort_by(self, f);
            }
            CmpAlgorithm::InsertionSortCmp => {
                insertion_sort_by(self, f);
            }
            CmpAlgorithm::SelectionSortCmp => {
                selection_sort_by(self, f);
            }
            CmpAlgorithm::QuickSortCmp => {
                quicksort_by(self, f);
            }
            CmpAlgorithm::MergeSortCmp => {
                *self = merge_sort_by(self, f);
            }
        }
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{
        bubble_sort_by, introsort, tests::test::random_vec, Algorithm, CmpAlgorithm, Sort,
    };

    #[test]
    fn test_vec_sort() {
        let mut vec = random_vec(100);
        let mut copy = vec.clone();

        vec.sort(Algorithm::IntroSort);
        introsort(&mut copy);

        assert_eq!(vec, copy);
    }

    #[test]
    fn test_vec_sort_by() {
        let mut vec = random_vec(100);
        let mut copy = vec.clone();

        vec.sort_by(CmpAlgorithm::BubbleSortCmp, |a, b| a.cmp(b));
        bubble_sort_by(&mut copy, |a, b| a.cmp(b));

        assert_eq!(vec, copy);
    }
}
