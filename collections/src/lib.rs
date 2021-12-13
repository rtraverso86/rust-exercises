// import and export public symbols
pub use ex_statistics::{Stats, compute_stats};

mod ex_statistics {
    use std::collections::HashMap;

    pub struct Stats {
        pub mean: f64,
        pub median: f64,
        pub mode: i32,
    }
    pub fn compute_stats(lst: &[i32]) -> Stats {
        let mut hash : HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;
        let len = lst.len();

        let mut sorted_lst = lst.to_vec();
        sorted_lst.sort_unstable();

        for elem in lst {
            let elem_count = hash.entry(*elem).or_insert(0);
            *elem_count += 1;
            sum += *elem;
        }

        Stats {
            mean: sum as f64 / len as f64,
            median: if len % 2 == 0 {
                (sorted_lst[len / 2 - 1] + sorted_lst[len / 2]) as f64 / 2.0
            } else {
                sorted_lst[len / 2] as f64
            },
            mode: *hash.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k).unwrap(),
        }
    }
}

#[cfg(test)]
mod chapter_8_exercises {
        use super::ex_statistics;

        #[test]
        fn test_1() {
            let myvec = vec![2, 2, 1, 2, 3];
            let stats = ex_statistics::compute_stats(&myvec);

            assert_eq!(stats.mean, 2.0);
            assert_eq!(stats.median, 2.0);
            assert_eq!(stats.mode, 2);
        }

        #[test]
        fn test_2() {
            let myvec = vec![9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25];
            let stats = ex_statistics::compute_stats(&myvec);

            assert_eq!(stats.mean, 16.75);
            assert_eq!(stats.median, 15.5);
            assert_eq!(stats.mode, 13);
        }

        #[test]
        fn test_3() {
            let myvec = vec![9, 1, 9, 9, 100];
            let stats = ex_statistics::compute_stats(&myvec);

            assert_eq!(stats.mean, 25.6);
            assert_eq!(stats.median, 9.0);
            assert_eq!(stats.mode, 9);
        }
}


#[cfg(test)]
mod hashmap_tests {
    use std::collections::HashMap;

    #[test]
    fn or_else() {
        let mut scores = HashMap::new();
        let blue = String::from("Blue");
        let yellow = String::from("Yellow");
        scores.insert(&blue, 10);

        scores.entry(&yellow).or_insert(50);
        let blue_2 = String::from("Blue");
        scores.entry(&blue_2).or_insert(50);

        println!("{:?}", scores);
        assert_eq!(*scores.get(&blue_2).unwrap(), 10);
        assert_eq!(*scores.get(&yellow).unwrap(), 50);
    }

}
