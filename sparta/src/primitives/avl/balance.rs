use std::cmp::max;

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
}
