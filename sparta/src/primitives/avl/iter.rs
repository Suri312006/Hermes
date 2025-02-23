use super::{AvlNode, AvlTree, AvlTreeSet};

#[derive(Debug)]
struct AvlTreeSetNodeIter<'a, T: Ord> {
    prev_nodes: Vec<&'a AvlNode<T>>,
    current_tree: &'a AvlTree<T>,
}

impl<'a, T: 'a + Ord> Iterator for AvlTreeSetNodeIter<'a, T> {
    type Item = &'a AvlNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => {
                        return None;
                    }

                    Some(prev_node) => {
                        self.current_tree = &prev_node.right;

                        return Some(&prev_node);
                    }
                },

                Some(ref curr_node) => {
                    if curr_node.left.is_some() {
                        self.prev_nodes.push(curr_node);
                        self.current_tree = &curr_node.left;

                        continue;
                    }

                    if curr_node.right.is_some() {
                        self.current_tree = &curr_node.right;
                        return Some(&curr_node);
                    }
                    self.current_tree = &None;

                    return Some(&curr_node);
                }
            }
        }
    }
}

impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.node_iter().map(|node| &node.value)
    }

    pub fn node_iter(&'a self) -> impl Iterator<Item = &'a AvlNode<T>> + 'a {
        AvlTreeSetNodeIter {
            prev_nodes: Vec::default(),
            current_tree: &self.root,
        }
    }
}

impl<T: Ord> FromIterator<T> for AvlTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();

        for i in iter {
            set.insert(i);
        }

        set
    }
}
