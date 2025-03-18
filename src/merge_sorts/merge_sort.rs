use std::cmp::Ordering;

pub fn merge_sort<T: Ord + Clone>(v: &mut [T]) -> Vec<T> {
    let l = v.len();
    if l < 2 {
        return v.to_vec();
    }

    let m = (l as f64 / 2.).floor() as usize;
    let (lp, rp) = v.split_at_mut(m);
    let lv = merge_sort(lp);
    let rv = merge_sort(rp);
    let lvl = lv.len();
    let rvl = rv.len();
    let mut li = 0;
    let mut ri = 0;
    let mut new_v = Vec::with_capacity(l);

    loop {
        if lv[li] > rv[ri] {
            new_v.push(rv[ri].clone());
            ri += 1;
        } else {
            new_v.push(lv[li].clone());
            li += 1;
        }
        if li == lvl {
            new_v.extend_from_slice(&rv[ri..]);
            break;
        }
        if ri == rvl {
            new_v.extend_from_slice(&lv[li..]);
            break;
        }
    }

    new_v
}

#[allow(unused_variables)]
pub fn merge_sort_by<T: Clone, F>(v: &mut [T], f: F) -> Vec<T>
where
    F: Fn(&T, &T) -> Ordering,
{
    unimplemented!("Missing implementation for `merge_sort_by()`")
}

#[cfg(test)]
mod tests {
    use crate::{merge_sort, not_in_place_test};
    //use crate::merge_sort_by;

    not_in_place_test!(merge_sort);
    //not_in_place_test!(merge_sort_by, |a, b| a.cmp(b));
}
