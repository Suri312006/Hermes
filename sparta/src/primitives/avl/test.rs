use quickcheck::Arbitrary;

use super::AvlTreeSet;

impl<T: Arbitrary + Ord> Arbitrary for AvlTreeSet<T> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let vec: Vec<T> = Arbitrary::arbitrary(g);
        vec.into_iter().collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let vec: Vec<T> = self.iter().cloned().collect();
        Box::new(vec.shrink().map(|v| v.into_iter().collect::<Self>()))
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::max, collections::BTreeSet};

    use crate::avl::{balance, AvlNode, AvlTreeSet};
    use itertools::{all, equal};
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_insert() {
        let mut set = AvlTreeSet::new();

        assert!(set.insert(1));
        assert!(!set.insert(1));
        assert!(set.insert(2));
    }

    #[test]
    fn iter() {
        let mut set = AvlTreeSet::new();

        for i in (1..4).rev() {
            set.insert(i);
        }

        let mut iter = set.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[quickcheck]
    fn node_height(set: AvlTreeSet<u16>) -> bool {
        all(set.node_iter(), |n| {
            n.height == 1 + max(n.left_height(), n.right_height())
        })
    }

    #[quickcheck]
    fn rotate_right_preserves_order(btree: BTreeSet<u8>) -> TestResult {
        let mut set = btree.iter().cloned().collect::<AvlTreeSet<_>>();

        if set.root.is_none() {
            return TestResult::discard();
        }

        if !set.root.as_mut().unwrap().rotate_right() {
            return TestResult::discard();
        }

        TestResult::from_bool(equal(set.iter(), btree.iter()))
    }
    #[quickcheck]
    fn rotate_left_preserves_order(btree: BTreeSet<u8>) -> TestResult {
        let mut set = btree.iter().cloned().collect::<AvlTreeSet<_>>();

        if set.root.is_none() {
            return TestResult::discard();
        }

        if !set.root.as_mut().unwrap().rotate_left() {
            return TestResult::discard();
        }

        TestResult::from_bool(equal(set.iter(), btree.iter()))
    }

    #[quickcheck]
    fn rotate_right_tils_balance_factor(xs: Vec<u32>) -> TestResult {
        let mut set = xs.iter().cloned().collect::<AvlTreeSet<_>>();

        if set.root.is_none() {
            return TestResult::discard();
        }

        let root_node = set.root.as_mut().unwrap();

        let balance_factor = root_node.balance_factor();

        if !root_node.rotate_right() {
            return TestResult::discard();
        }

        let tilted_factor = root_node.balance_factor();

        TestResult::from_bool(balance_factor - tilted_factor == 2)
    }
}

#[cfg(test)]
mod properties {

    use crate::avl::AvlTreeSet;

    use itertools::equal;
    use quickcheck_macros::quickcheck;

    use std::collections::BTreeSet;

    #[quickcheck]
    fn iterator_parity(xs: Vec<usize>) -> bool {
        let avl_set = xs.iter().cloned().collect::<AvlTreeSet<usize>>();
        let btree_set = xs.iter().cloned().collect::<BTreeSet<usize>>();

        equal(avl_set.iter(), btree_set.iter())
    }
    #[quickcheck]
    fn insert_parity(mut btree_set: BTreeSet<u8>, x: u8) -> bool {
        let mut avl_set = btree_set.iter().cloned().collect::<AvlTreeSet<_>>();
        avl_set.insert(x) == btree_set.insert(x)
    }
}
