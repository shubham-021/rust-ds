#![allow(unused)]

// Balance_factor(node) = height(left_subtree) - height(right_subtree)
// for every node: -1 <= Balance_factor(node) <= 1

use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    value: T,
    height: i32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }
}

struct AvlTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Debug> AvlTree<T> {
    fn new(value: T) -> Self {
        Self {
            root: Some(Box::new(Node::new(value))),
        }
    }

    fn insert_loop(&mut self, value: T) {
        let mut current = &mut self.root;
        loop {
            match current {
                Some(node) => {
                    if &value <= &node.value {
                        current = &mut node.left;
                    } else {
                        current = &mut node.right;
                    }
                }

                None => {
                    *current = Some(Box::new(Node::new(value)));
                    return;
                    // return Some(current);
                }
            }
        }
    }

    fn insert(&mut self, value: T) {
        self.root = Self::insert_rec_helper(self.root.take(), value);
    }

    fn insert_rec_helper(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        match node {
            None => Some(Box::new(Node::new(value))),
            Some(mut node) => {
                if &value <= &node.value {
                    node.left = Self::insert_rec_helper(node.left, value);
                } else {
                    node.right = Self::insert_rec_helper(node.right, value);
                }

                Self::update_height(&mut node);

                let balance = Self::balance_factor(&node);

                if balance > 1 {
                    let left_balance = Self::balance_factor(node.left.as_ref().unwrap());
                    if left_balance >= 0 {
                        // LL case
                        return Some(Self::right_rotate(node));
                    } else {
                        // LR case
                        node.left = Some(Self::left_rotate(node.left.unwrap()));
                        return Some(Self::right_rotate(node));
                    }
                } else if balance < -1 {
                    let right_balance = Self::balance_factor(node.right.as_ref().unwrap());
                    if right_balance <= 0 {
                        // RR case
                        return Some(Self::left_rotate(node));
                    } else {
                        // RL case
                        node.right = Some(Self::right_rotate(node.right.unwrap()));
                        return Some(Self::left_rotate(node));
                    }
                }

                Some(node)
            }
        }
    }

    fn right_rotate(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut left = node.left.take().unwrap();
        let right_subtree = left.right.take();
        node.left = right_subtree;
        left.right = Some(node);
        left
    }

    fn left_rotate(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut right = node.right.take().unwrap();
        let left_subtree = right.left.take();
        node.right = left_subtree;
        right.left = Some(node);
        right
    }

    fn inorder_helper(node: &Node<T>) {
        let left = &node.left;
        if left.is_some() {
            Self::inorder_helper(node.left.as_ref().unwrap());
        }
        println!("value: {:?}, height: {:?} ", node.value, node.height);

        let right = &node.right;
        if right.is_some() {
            Self::inorder_helper(node.right.as_ref().unwrap());
        }
    }

    fn inorder(&self) {
        if let Some(node) = &self.root {
            Self::inorder_helper(node);
        }

        // Err("Invalid root")
    }

    fn height_helper(node: &Option<Box<Node<T>>>, count: i32) -> Option<i32> {
        if let Some(node) = node {
            let count_left = Self::height_helper(&node.left, count + 1);

            let count_right = Self::height_helper(&node.right, count + 1);

            match (count_left, count_right) {
                (Some(left), Some(right)) => {
                    if left > right {
                        return Some(left);
                    } else {
                        return Some(right);
                    }
                }

                (Some(left), None) => {
                    return Some(left);
                }

                (None, Some(right)) => {
                    return Some(right);
                }

                (None, None) => {
                    let this_count = count;
                    return Some(this_count);
                }
            }
        }

        None
    }

    fn prev_height(node: &Option<Box<Node<T>>>) -> Option<i32> {
        let current = node.as_ref()?;
        if current.left.is_none() && current.right.is_none() {
            return Some(1);
        }

        let count = 1;

        Self::height_helper(node, count)
    }

    fn height_rec(node: &Option<Box<Node<T>>>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                let left_height = Self::height(&node.left);
                let right_height = Self::height(&node.right);

                1 + left_height.max(right_height)
            }
        }
    }

    fn height(node: &Option<Box<Node<T>>>) -> i32 {
        match node {
            None => 0,
            Some(node) => node.height,
        }
    }

    fn balance_factor(node: &Node<T>) -> i32 {
        Self::height(&node.left) - Self::height(&node.right)
    }

    fn update_height(node: &mut Box<Node<T>>) {
        let left_height = Self::height(&node.left);
        let right_height = Self::height(&node.right);

        node.height = 1 + left_height.max(right_height);
    }
}

fn main() {
    let mut new_avl_tree = AvlTree::new(10);
    new_avl_tree.insert(5);
    new_avl_tree.insert(3);
    new_avl_tree.insert(8);
    new_avl_tree.insert(9);
    new_avl_tree.insert(20);
    new_avl_tree.insert(15);
    new_avl_tree.insert(50);

    new_avl_tree.inorder();

    // let height = AvlTree::height(&new_avl_tree.root);
    // println!("\nHeight of root is: {}", height);
}
