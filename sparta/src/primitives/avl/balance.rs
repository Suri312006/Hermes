use std::{
    cmp::max,
    mem::{replace, swap},
};

use super::AvlNode;

impl<'a, T: 'a + Ord> AvlNode<T> {
    pub fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height())
    }
    pub fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height())
    }

    pub fn balance_factor(&self) -> isize {
        let left_height = self.left_height() as isize;

        let right_height = self.right_height() as isize;

        left_height - right_height
    }

    pub fn height(&self) -> usize {
        1 + max(
            self.left.as_ref().map_or(0, |node| node.height()),
            self.right.as_ref().map_or(0, |node| node.height()),
        )
    }

    pub fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    pub fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            return false;
        }

        let left_node = self.left.as_mut().unwrap();
        let left_right_tree = left_node.right.take();
        let left_left_tree = left_node.left.take();

        let mut new_right_tree = replace(&mut self.left, left_left_tree);

        swap(&mut self.value, &mut new_right_tree.as_mut().unwrap().value);

        let right_tree = self.right.take();

        let new_right_node = new_right_tree.as_mut().unwrap();
        new_right_node.left = left_right_tree;
        new_right_node.right = right_tree;

        self.right = new_right_tree;

        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }

        self.update_height();
        true
    }

    pub fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }

        let right_node = self.right.as_mut().unwrap();
        let right_right_tree = right_node.right.take();
        let right_left_tree = right_node.left.take();

        let mut new_left_tree = replace(&mut self.right, right_right_tree);
        swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().value);

        let left_tree = self.left.take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.right = right_left_tree;
        new_left_node.left = left_tree;

        self.left = new_left_tree;

        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }

        self.update_height();
        true
    }
}
