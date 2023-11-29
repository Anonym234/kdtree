use std::cmp::Ordering;

pub fn hoare_a<T>(data: &mut Vec<T>, key_cmp: impl Fn(&T, &T) -> Ordering) -> usize {
    fn recurse<T>(
        data: &mut Vec<T>,
        target_idx: usize,
        left: usize,
        right: usize,
        key_cmp: impl Fn(&T, &T) -> Ordering,
    ) -> usize {
        assert!(left <= target_idx && target_idx <= right);
        if left == right {
            return target_idx;
        }
        assert!(left < right);

        let mut pivot_idx = (right - left) / 2 + left;
        assert!((left..=right).contains(&pivot_idx));
        let mut l = left;
        let mut r = right;

        // somewhat hoare's partition scheme
        loop {
            // l == pivot | l > pivot | r == pivot | r < pivot | can happen | swap ? | new pivot idx | implemented
            // ---
            // 0          | 0         | 0          | 0         | no         |        |               |
            // 0          | 0         | 0          | 1         | no         |        |               |
            // 0          | 0         | 1          | 0         | no         |        |               |
            // 0          | 0         | 1          | 1         | no         |        |               |
            // 0          | 1         | 0          | 0         | no         |        |               |
            // 0          | 1         | 0          | 1         | yes        | yes    | last          | no
            // 0          | 1         | 1          | 0         | yes        | yes    | l             | no
            // 0          | 1         | 1          | 1         | no         |        |               |
            // 1          | 0         | 0          | 0         | yes        | no     | l             | yes
            // 1          | 0         | 0          | 1         | yes        | yes    | r             | no
            // 1          | 0         | 1          | 0         | yes        | yes    | l             | no
            // 1          | 0         | 1          | 1         | no         |        |               |
            // 1          | 1         | 0          | 0         | no         |        |               |
            // 1          | 1         | 0          | 1         | no         |        |               |
            // 1          | 1         | 1          | 0         | no         |        |               |
            // 1          | 1         | 1          | 1         | no         |        |               |

            // assert!((l..=r).contains(&pivot_idx));

            #[derive(Debug)]
            enum State {
                EQ,
                LTorGT,
                OutOfRange,
            }

            let l_state = loop {
                if l >= r {
                    break State::OutOfRange;
                }

                match key_cmp(&data[l], &data[pivot_idx]) {
                    Ordering::Less => l += 1,
                    Ordering::Equal => break State::EQ,
                    Ordering::Greater => break State::LTorGT,
                }
            };

            let r_state = loop {
                if r <= l {
                    break State::OutOfRange;
                }

                match key_cmp(&data[r], &data[pivot_idx]) {
                    Ordering::Less => break State::LTorGT,
                    Ordering::Equal => break State::EQ,
                    Ordering::Greater => r -= 1,
                }
            };

            let (swap, next_pivot, l_step, r_step) = match (l_state, r_state) {
                (State::OutOfRange, _) | (_, State::OutOfRange) => break,
                (State::EQ, State::EQ) => (true, usize::min(l, pivot_idx), 0, 1),
                (State::EQ, State::LTorGT) => (true, usize::min(r, pivot_idx), 1, 0),
                (State::LTorGT, State::EQ) => (true, usize::min(l, pivot_idx), 0, 1),
                (State::LTorGT, State::LTorGT) => (true, pivot_idx, 1, 1),
            };

            if swap {
                data.swap(l, r);
            }
            pivot_idx = next_pivot;
            l += l_step;
            r -= r_step;
        }

        match key_cmp(&data[target_idx], &data[pivot_idx]) {
            Ordering::Less => recurse(data, target_idx, left, pivot_idx - 1, key_cmp),
            Ordering::Equal => {
                assert!(pivot_idx <= target_idx);
                pivot_idx
            }
            Ordering::Greater => recurse(data, target_idx, pivot_idx + 1, right, key_cmp),
        }
    }

    recurse(data, data.len() / 2, 0, data.len() - 1, key_cmp)
}

pub fn hoare_b<T>(data: &mut Vec<T>, key_cmp: impl Fn(&T, &T) -> Ordering) -> usize {
    fn recurse<T>(
        data: &mut Vec<T>,
        left: usize,
        target_idx: usize,
        right: usize,
        key_cmp: impl Fn(&T, &T) -> Ordering,
    ) -> usize {
        assert!(left < right);
        assert!(right < data.len());
        assert!((left..=right).contains(&target_idx));

        let mut pivot_idx = (right - left) / 2 + left;
        assert!((left..=right).contains(&pivot_idx));

        let mut l = left;
        let mut r = right;

        loop {
            // step l
            while l < r {
                match key_cmp(&data[l], &data[pivot_idx]) {
                    Ordering::Less => l += 1,
                    Ordering::Equal => {
                        pivot_idx = usize::min(l, pivot_idx);
                        break;
                    }
                    Ordering::Greater => break,
                }
            }

            // step r
            while l < r {
                match key_cmp(&data[r], &data[pivot_idx]) {
                    Ordering::Less => break,
                    Ordering::Equal => {
                        pivot_idx = usize::min(r, pivot_idx);
                        r -= 1
                    }
                    Ordering::Greater => r -= 1,
                }
            }

            if l >= r {
                break;
            }

            data.swap(l, r);
            if r == pivot_idx {
                pivot_idx = l;
            } else if l == pivot_idx {
                pivot_idx = r;
            }
        }

        if key_cmp(&data[pivot_idx], &data[target_idx]).is_eq() {
            pivot_idx
        } else {
            match usize::cmp(&target_idx, &pivot_idx) {
                Ordering::Less => recurse(data, left, target_idx, pivot_idx - 1, key_cmp),
                Ordering::Equal => unreachable!(),
                Ordering::Greater => recurse(data, pivot_idx + 1, target_idx, right, key_cmp),
            }
        }
    }

    recurse(data, 0, data.len() / 2, data.len() - 1, key_cmp)
}
