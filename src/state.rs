use std::cmp::{Eq, Ordering};

pub mod state {
    #[derive(Copy, Clone, PartialEq)]
    struct State {
        cost: f64,
        position: usize,
    }

    impl Eq for State {}

    impl Ord for State {
        fn cmp(&self, other: &State) -> Ordering {
            // flip the ordering of other and self to turn the heap into a min-heap
            // the then_with part is necessary to make implementation consistent with PartialEq
            other
                .cost
                .partial_cmp(&self.cost)
                .unwrap_or(Ordering::Equal)
                .then_with(|| self.position.cmp(&other.position))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &State) -> Option<Ordering> {
            Some(self.cmp(&other))
        }
    }
}
