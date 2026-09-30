#![allow(unused)]

use std::{cmp::Ordering, fmt::Debug, io::ErrorKind::StaleNetworkFileHandle, thread::current};

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    fn new(value: T) -> Self {
        Self {
            root: Some(Box::new(Node::new(value))),
        }
    }

    // fn insert(&mut self, value: T) {
    //     let mut current = &mut self.root;
    //
    //     loop {
    //         if current.is_none() {
    //             *current = Some(Box::new(Node::new(value)));
    //             return;
    //         }
    //
    //         let node = current.as_mut().unwrap();
    //         if value < node.value {
    //             current = &mut node.left;
    //         } else {
    //             current = &mut node.right;
    //         }
    //     }
    // }

    fn insert(&mut self, value: T) {
        Self::insert_helper(&mut self.root, value);
    }

    fn insert_helper(node: &mut Option<Box<Node<T>>>, value: T) {
        match node {
            None => {
                *node = Some(Box::new(Node::new(value)));
            }

            Some(current) if value < current.value => {
                Self::insert_helper(&mut current.left, value);
            }

            Some(current) => {
                Self::insert_helper(&mut current.right, value);
            }
        }
    }

    fn contains(&self, value: &T) -> bool {
        // let mut current = &self.root;
        // loop {
        //     match current {
        //         None => return false,
        //         Some(node) if value == &node.value => {
        //             return true;
        //         }
        //         Some(node) if value < &node.value => {
        //             current = &node.left;
        //         }
        //         Some(node) => {
        //             current = &node.right;
        //         }
        //     }
        // }

        let mut current = self.root.as_deref();
        while let Some(node) = current {
            if value == &node.value {
                return true;
            }

            current = if value < &node.value {
                node.left.as_deref()
            } else {
                node.right.as_deref()
            };
        }

        false
    }

    fn remove_one_child_node(node: &mut Option<Box<Node<T>>>) -> bool {
        if let Some(curr_node) = node {
            if curr_node.left.is_some() {
                let child_node = curr_node.left.take();
                *node = child_node;
                return true;
            } else if curr_node.right.is_some() {
                let child_node = curr_node.right.take();
                *node = child_node;
                return true;
            }
        }

        false
    }

    fn inorder_predecessor(node: &mut Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let current = node.as_mut()?;
        if current.right.is_none() {
            let mut predecessor = node.take().unwrap();
            *node = predecessor.left.take();
            return Some(predecessor);
        }

        Self::inorder_predecessor(&mut current.right)
    }

    fn remove_two_child_node(node: &mut Option<Box<Node<T>>>) -> bool {
        let mut current = node.take().unwrap();
        let mut predecessor = Self::inorder_predecessor(&mut current.left).unwrap();

        current.value = predecessor.value;

        *node = Some(current);
        true
    }

    fn remove(&mut self, value: &T) -> bool {
        let mut current = &mut self.root;
        loop {
            match current {
                None => {
                    return false;
                }

                Some(node) if value == &node.value => {
                    if node.left.is_some() && node.right.is_some() {
                        return Self::remove_two_child_node(current);
                    } else if node.left.is_none() && node.right.is_none() {
                        current.take();
                        return true;
                    } else {
                        return Self::remove_one_child_node(current);
                    }
                }

                Some(node) => {
                    current = if value < &node.value {
                        &mut node.left
                    } else {
                        &mut node.right
                    };
                }
            }
        }
    }
}

impl<T: Debug> BinarySearchTree<T> {
    fn preorder_helper(node: &Node<T>) {
        print!("{:?} ", node.value);
        if let Some(left) = node.left.as_ref() {
            Self::preorder_helper(left);
        }

        if let Some(right) = node.right.as_ref() {
            Self::preorder_helper(right);
        }
    }

    fn preorder(&self) {
        if let Some(node) = self.root.as_ref() {
            Self::preorder_helper(node);
        }
    }

    fn inorder_helper(node: &Node<T>) {
        if let Some(left) = node.left.as_ref() {
            Self::inorder_helper(left);
        }

        print!("{:?} ", node.value);

        if let Some(right) = node.right.as_ref() {
            Self::inorder_helper(right);
        }
    }

    fn inorder(&self) {
        if let Some(node) = self.root.as_ref() {
            Self::inorder_helper(node);
        }
    }

    fn postorder_helper(node: &Node<T>) {
        if let Some(left) = node.left.as_ref() {
            Self::postorder_helper(left);
        }

        if let Some(right) = node.right.as_ref() {
            Self::postorder_helper(right);
        }

        print!("{:?} ", node.value);
    }

    fn postorder(&self) {
        if let Some(node) = self.root.as_ref() {
            Self::postorder_helper(node);
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::new(50);
    tree.insert(30);
    tree.insert(70);
    tree.insert(20);
    tree.insert(10);
    tree.insert(90);

    print!("PreOrder: ");
    tree.preorder();
    println!();
    print!("InOrder: ");
    tree.inorder();
    println!();
    print!("PostOrder: ");
    tree.postorder();

    println!();
    let a = 30;
    println!("Contains 30? {:?}", tree.contains(&a));
    let b = 10;
    println!("Contains 10? {:?}", tree.contains(&b));
    let c = 100;
    println!("Contains 100? {:?}", tree.contains(&c));
}
