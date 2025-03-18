mod bubble_sort;
pub mod quicksort; // public module should be avoided

pub use bubble_sort::{bubble_sort, bubble_sort_by};
pub use quicksort::{quicksort, quicksort_by};
