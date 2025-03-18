#[cfg(test)]
pub mod tests;

mod exchange_sorts;
mod hybrid_sorts;
mod insertion_sorts;
mod merge_sorts;
mod selection_sorts;
mod sort;

pub use exchange_sorts::{bubble_sort, bubble_sort_by, quicksort, quicksort_by};
pub use hybrid_sorts::{introsort, introsort_by};
pub use insertion_sorts::{insertion_sort, insertion_sort_by};
pub use merge_sorts::{merge_sort, merge_sort_by};
pub use selection_sorts::{heapsort, heapsort_by, selection_sort, selection_sort_by};
pub use sort::{Algorithm, CmpAlgorithm, Sort};
