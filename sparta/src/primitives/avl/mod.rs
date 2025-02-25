use std::cmp::Ordering;
mod balance;
mod iter;
mod test;

// building it asa set
//
#[derive(Debug, PartialEq, Clone)]
// our value inside must be able to be orderd, so we condition it to have this property
struct AvlNode<T: Ord> {
    value: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
    height: usize,
}

type AvlTree<T> = Option<Box<AvlNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<T: Ord> AvlTreeSet<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T) -> bool {
        let mut curr_tree = &mut self.root;
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::new();

        while let Some(curr_node) = curr_tree {
            prev_ptrs.push(&mut **curr_node);
            match curr_node.value.cmp(&value) {
                Ordering::Less => curr_tree = &mut curr_node.right,
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => curr_tree = &mut curr_node.left,
            }
        }

        *curr_tree = Some(Box::new(AvlNode {
            value,
            left: None,
            right: None,
            height: 1,
        }));

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
        }
        true
    }
}
