#[macro_export]
#[cfg(test)]
macro_rules! in_place_test {
    ($func: ident) => {
        paste::paste! {
            #[test]
            fn [<test_ $func _random>]() {
                let mut data = $crate::tests::test::random_vec(100);
                let mut copy = data.clone();
                $func(&mut data);
                copy.sort();
                assert_eq!(data, copy);
            }

            #[test]
            fn [<test_ $func _decreasing>]() {
                let mut data: Vec<i32> = (-50..50).rev().collect();
                let mut copy = data.clone();
                $func(&mut data);
                copy.sort();
                assert_eq!(data, copy);
            }

            #[test]
            fn [<test_ $func _sorted>]() {
                let mut data: Vec<i32> = (-50..50).collect();
                let copy = data.clone();
                $func(&mut data);
                assert_eq!(data, copy);
            }
        }
    };
    ($func: ident, $cmp: expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $func _random>]() {
                let mut data = $crate::tests::test::random_vec(100);
                let mut copy = data.clone();
                $func(&mut data, $cmp);
                copy.sort();
                assert_eq!(data, copy);
            }

            #[test]
            fn [<test_ $func _decreasing>]() {
                let mut data: Vec<i32> = (-50..50).rev().collect();
                let mut copy = data.clone();
                $func(&mut data, $cmp);
                copy.sort();
                assert_eq!(data, copy);
            }

            #[test]
            fn [<test_ $func _sorted>]() {
                let mut data: Vec<i32> = (-50..50).collect();
                let copy = data.clone();
                $func(&mut data, $cmp);
                assert_eq!(data, copy);
            }
        }
    };
}

#[macro_export]
#[cfg(test)]
macro_rules! not_in_place_test {
    ($func: ident) => {
        paste::paste! {
            #[test]
            fn [<test_ $func _random>]() {
                let mut data = $crate::tests::test::random_vec(100);
                let mut copy = data.clone();
                let res = $func(&mut data);
                copy.sort();
                assert_eq!(res, copy);
            }

            #[test]
            fn [<test_ $func _decreasing>]() {
                let mut data: Vec<i32> = (-50..50).rev().collect();
                let mut copy = data.clone();
                let res = $func(&mut data);
                copy.sort();
                assert_eq!(res, copy);
            }

            #[test]
            fn [<test_ $func _sorted>]() {
                let mut data: Vec<i32> = (-50..50).collect();
                let copy = data.clone();
                let res = $func(&mut data);
                assert_eq!(res, copy);
            }
        }
    };
    ($func: ident, $cmp: expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $func _random>]() {
                let mut data = $crate::tests::test::random_vec(100);
                let mut copy = data.clone();
                let res = $func(&mut data, $cmp);
                copy.sort();
                assert_eq!(res, copy);
            }

            #[test]
            fn [<test_ $func _decreasing>]() {
                let mut data: Vec<i32> = (-50..50).rev().collect();
                let mut copy = data.clone();
                let res = $func(&mut data, $cmp);
                copy.sort();
                assert_eq!(res, copy);
            }

            #[test]
            fn [<test_ $func _sorted>]() {
                let mut data: Vec<i32> = (-50..50).collect();
                let copy = data.clone();
                let res = $func(&mut data, $cmp);
                assert_eq!(data, copy);
            }
        }
    };
}

#[cfg(test)]
pub mod test {
    use rand::prelude::*;
    use std::time::Instant;

    pub fn random_vec(size: usize) -> Vec<i32> {
        let mut vec: Vec<i32> = vec![];
        let mut rng = rand::rng();
        for _ in 0..size {
            vec.push(rng.random());
        }
        vec.into_iter().take(size).collect()
    }

    #[allow(dead_code)]
    fn compare_algorithms<F, G>(sort1: F, sort2: G)
    where
        F: Fn(&mut [i32]) -> &mut [i32],
        G: Fn(&mut [i32]) -> &mut [i32],
    {
        let mut res = vec![];
        let mut res2 = vec![];

        for _ in 0..100 {
            let mut random_vec = random_vec(1000);
            let mut random_vec2 = random_vec.clone();

            let start = Instant::now();
            sort1(&mut random_vec);
            let v1 = start.elapsed();

            let start = Instant::now();
            sort2(&mut random_vec2);
            let v2 = start.elapsed();

            res.push(v1.as_nanos());
            res2.push(v2.as_nanos());
        }

        println!("v1 => {}", res.iter().sum::<u128>() / res.len() as u128);
        println!("v2 => {}", res2.iter().sum::<u128>() / res2.len() as u128);

        let mut random_vec = (0..200).collect::<Vec<i32>>();
        let mut random_vec2 = random_vec.clone();

        let start = Instant::now();
        sort1(&mut random_vec);
        let v1 = start.elapsed();

        let start = Instant::now();
        sort2(&mut random_vec2);
        let v2 = start.elapsed();

        println!("sorted v1 => {}", v1.as_nanos());
        println!("sorted v2 => {}", v2.as_nanos());
    }
}
