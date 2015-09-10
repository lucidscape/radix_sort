#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use rand::Rng;
    use rand::XorShiftRng;
    use test::Bencher;
    use test::black_box;
    use super::super::*;

    #[derive(Default, Clone)]
    pub struct Item {
        idx: u64
    }

    /// As required for radix sorting
    impl Indexed for Item {
        fn get_index(&self) -> u64 {
            self.idx
        }
    }

    /// Required for std/sort baseline benchmark
    impl PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            self.idx == other.idx
        }
    }

    /// Required for std/sort baseline benchmark
    impl Eq for Item {
    }

    /// Required for std/sort baseline benchmark
    impl PartialOrd for Item {
        fn lt(&self, other: &Self) -> bool {
            self.idx < other.idx
        }

        fn le(&self, other: &Self) -> bool {
            self.idx <= other.idx
        }

        fn gt(&self, other: &Self) -> bool {
            self.idx > other.idx
        }

        fn ge(&self, other: &Self) -> bool {
            self.idx >= other.idx
        }

        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.idx.cmp(&other.idx))
        }
    }

    /// Required for std/sort baseline benchmark
    impl Ord for Item {
        fn cmp(&self, other: &Self) -> Ordering {
            self.idx.cmp(&other.idx)
        }
    }

    const COUNT:usize = 100000;

    #[test]
    fn test_sort() {
        let mut rnd = XorShiftRng::new_unseeded();
        let mut values = Vec::new();
        for _ in 0..COUNT {
            values.push(Item { idx: rnd.next_u64() } );
        }

        let observed = sort(&values);
        for i in 1..COUNT {
            assert! (observed[i - 1].idx <= observed[i].idx);
        }
    }

    #[bench]
    fn bench_radix_sort(b: &mut Bencher) {
        let mut rnd = XorShiftRng::new_unseeded();
        let mut values = Vec::new();
        for _ in 0..COUNT {
            values.push(Item { idx: rnd.next_u64() } );
        }

        b.iter(|| {
            let values = values.clone();
            black_box(sort(&values));
        });
    }

    #[bench]
    fn bench_std_sort_baseline(b: &mut Bencher) {
        let mut rnd = XorShiftRng::new_unseeded();
        let mut values = Vec::new();
        for _ in 0..COUNT {
            values.push(Item { idx: rnd.next_u64() } );
        }

        b.iter(|| {
            let mut values = values.clone();
            values.sort();
        });
    }
}
