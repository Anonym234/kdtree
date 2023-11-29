use std::{cmp::Ordering, fmt::Debug};

pub trait KDPoint {
    type Key: Ord;
    type Distance: Ord;

    fn kdkey(&self, dimension: usize) -> Self::Key;
    fn distance(lhs: &Self, rhs: &Self) -> Self::Distance;
    fn key_distance(lhs: &Self::Key, rhs: &Self::Key) -> Self::Distance;
}

mod points;
use points::*;

fn compare_element<E: KDPoint>(left: &E, right: &E, dimension: usize) -> Ordering {
    E::Key::cmp(&left.kdkey(dimension), &right.kdkey(dimension))
}

fn make_compare<E: KDPoint>(dimension: usize) -> impl Fn(&E, &E) -> Ordering {
    move |l: &E, r: &E| compare_element(l, r, dimension)
}

mod partition_functions;
use partition_functions::hoare_b as partition;

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl<T: KDPoint> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    fn make(mut data: Vec<T>, dimension: usize) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let idx = partition(&mut data, make_compare(dimension));
        assert!(idx < data.len());

        let mut right = data.split_off(idx);
        assert!(!right.is_empty());

        let element = right.swap_remove(0);
        let left = data;
        let right = right;

        let left = Self::make(left, dimension + 1);
        let right = Self::make(right, dimension + 1);

        Some(Self {
            data: element,
            left: left.map(Box::new),
            right: right.map(Box::new),
        })
    }

    fn insert(&mut self, data: T, dimension: usize) {
        let selfkey = self.data.kdkey(dimension);
        let datakey = data.kdkey(dimension);

        let child = if selfkey < datakey {
            &mut self.left
        } else {
            &mut self.right
        };

        if let Some(child) = child {
            child.insert(data, dimension + 1);
        } else {
            *child = Some(Box::new(Self::new(data)));
        }
    }
}

trait Visitor<'t, T> {
    fn dimension(&self) -> usize;
    fn inc_dimension(&mut self);
    fn dec_dimension(&mut self);

    fn visit(&mut self, node: &'t Node<T>);

    fn visit_left(&mut self, node: &'t Node<T>) {
        if let Some(child) = &node.left {
            self.inc_dimension();
            self.visit(child);
            self.dec_dimension();
        }
    }
    fn visit_right(&mut self, node: &'t Node<T>) {
        if let Some(child) = &node.right {
            self.inc_dimension();
            self.visit(child);
            self.dec_dimension();
        }
    }
}

trait MutVisitor<'t, T>: Visitor<'t, T> {
    fn visit_mut(&mut self, node: &'t mut Node<T>);

    fn visit_left_mut(&mut self, node: &'t mut Node<T>) {
        if let Some(child) = &mut node.left {
            self.inc_dimension();
            self.visit_mut(child);
            self.dec_dimension();
        }
    }
    fn visit_right_mut(&mut self, node: &'t mut Node<T>) {
        if let Some(child) = &mut node.right {
            self.inc_dimension();
            self.visit_mut(child);
            self.dec_dimension();
        }
    }
}

#[derive(Debug)]
pub struct KDTree<T> {
    root: Option<Node<T>>,
}

impl<T: KDPoint> KDTree<T> {
    pub fn make(data: Vec<T>) -> Self {
        Self {
            root: Node::make(data, 0),
        }
    }

    /// insert new point, might unbalance the tree
    pub fn insert(&mut self, data: T) {
        if let Some(root) = &mut self.root {
            root.insert(data, 0);
        } else {
            self.root = Some(Node::new(data))
        }
    }

    pub fn find_nearest(&self, search: &T) -> Option<&T> {
        struct Vizz<'t, 's, T: KDPoint> {
            dimension: usize,
            best: Option<&'t T>,
            distance: Option<T::Distance>,
            search: &'s T,
        }

        impl<'t, 's, T: KDPoint> Vizz<'t, 's, T> {
            fn new(search: &'s T) -> Self {
                Self {
                    dimension: 0,
                    best: None,
                    distance: None,
                    search,
                }
            }

            fn cmp(&self, lhs: &T, rhs: &T) -> Ordering {
                T::Key::cmp(&lhs.kdkey(self.dimension), &rhs.kdkey(self.dimension))
            }
        }

        impl<'t, 's, T: KDPoint> Visitor<'t, T> for Vizz<'t, 's, T> {
            fn dimension(&self) -> usize {
                self.dimension
            }

            fn inc_dimension(&mut self) {
                self.dimension += 1;
            }

            fn dec_dimension(&mut self) {
                self.dimension -= 1;
            }

            fn visit(&mut self, node: &'t Node<T>) {
                let [first, second] = if self.cmp(self.search, &node.data).is_lt() {
                    [Self::visit_left, Self::visit_right]
                } else {
                    [Self::visit_right, Self::visit_left]
                };

                // traverse first child
                first(self, node);

                // check if current is better
                let curr_dist = T::distance(self.search, &node.data);
                if self
                    .distance
                    .as_ref()
                    .map(|best_dist| curr_dist < *best_dist)
                    .unwrap_or(true)
                {
                    self.best = Some(&node.data);
                    self.distance = Some(curr_dist);
                }

                // check if need for traversal into second child
                let target_to_split = T::key_distance(
                    &self.search.kdkey(self.dimension),
                    &node.data.kdkey(self.dimension),
                );

                // if current best "range" is wrapping over to other side of split, traverse other child
                if *self.distance.as_ref().unwrap() > target_to_split {
                    second(self, node);
                }
            }
        }

        let mut visitor = Vizz::new(search);
        if let Some(root) = &self.root {
            visitor.visit(root);
        }
        visitor.best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn main() {
        use rand::random;

        let data: Vec<Point3D<F64>> = (0..10)
            .map(|_| [random::<f64>(), random::<f64>(), random::<f64>()].into())
            .collect();

        println!("{data:#?}");

        let tree = KDTree::make(data);

        println!("{tree:#?}");
    }

    // #[cfg(öksdf)]
    mod partition {
        use super::*;

        fn check<T: Debug + Ord>(data: &Vec<T>, claimed_median_idx: usize) {
            assert!(claimed_median_idx < data.len());

            for i in 0..claimed_median_idx {
                assert!(dbg!(&data[i]) < dbg!(&data[claimed_median_idx]));
            }
            for i in claimed_median_idx..data.len() {
                assert!(dbg!(&data[i]) >= dbg!(&data[claimed_median_idx]));
            }

            assert!(data[claimed_median_idx..].len() >= data[..claimed_median_idx].len())
        }

        #[test]
        fn single() {
            let mut data = vec![17];
            let idx = partition(&mut data, u32::cmp);
            check(&data, idx);
            assert_eq!(idx, 0);
        }

        #[test]
        fn pair() {
            let mut data = vec![1, 2];
            let idx = partition(&mut data, u32::cmp);

            assert_eq!(idx, 1);
        }

        #[test]
        fn unique() {
            let mut data = (0..10).collect();
            let idx = partition(&mut data, u32::cmp);
            check(&data, idx);

            let mut data = (0..11).collect();
            let idx = partition(&mut data, u32::cmp);
            check(&data, idx);

            let mut data = (0..12).collect();
            let idx = partition(&mut data, u32::cmp);
            check(&data, idx);
        }

        #[test]
        fn sames() {
            let mut data = (0..=32)
                .map(|x| [x, x, x, x].into_iter())
                .flatten()
                .collect();
            let idx = partition(&mut data, u32::cmp);
            check(&data, idx);
        }

        #[test]
        fn random_data() {
            use rand::random;
            let mut data = (0..=random::<usize>() % 100)
                .map(|_| random::<u32>() % 10)
                .collect();
            let idx = partition(&mut data, u32::cmp);
            check(&data, idx);
        }
    }
}
