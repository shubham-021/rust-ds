#![allow(unused)]

use std::fmt::Debug;

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

    fn set_left(&mut self, value: T) {
        self.left = Some(Box::new(Node::new(value)));
    }

    fn set_right(&mut self, value: T) {
        self.right = Some(Box::new(Node::new(value)));
    }
}

#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Debug> BinaryTree<T> {
    fn new(value: T) -> Self {
        Self {
            root: Some(Box::new(Node::new(value))),
        }
    }

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
        if let Some(root) = self.root.as_ref() {
            Self::preorder_helper(root);
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
        if let Some(root) = self.root.as_ref() {
            Self::inorder_helper(root);
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
        if let Some(root) = self.root.as_ref() {
            Self::postorder_helper(root);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new(10);

    // if let Some(root) = tree.root.as_mut() {
    //     root.left = Some(Box::new(Node::new(20)));
    //     root.right = Some(Box::new(Node::new(30)));
    //
    //     if let Some(root) = root.left.as_mut() {
    //         root.left = Some(Box::new(Node::new(40)));
    //         root.right = Some(Box::new(Node::new(50)));
    //     }
    // }

    if let Some(root) = tree.root.as_mut() {
        root.set_left(20);
        root.set_right(30);

        if let Some(root) = root.left.as_mut() {
            root.set_left(40);
            root.set_right(50);
        }
    }

    print!("Preorder: ");
    tree.preorder();
    println!();
    print!("Inorder: ");
    tree.inorder();
    println!();
    print!("Postorder: ");
    tree.postorder();

    // println!("{:?}", tree);
}
