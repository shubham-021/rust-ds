#![allow(unused)]
use std::collections::VecDeque;

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

struct QueueVecD<T> {
    data: VecDeque<T>,
}

impl<T> QueueVecD<T> {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, value: T) {
        self.data.push_back(value);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        self.data.front()
    }
}

struct QueueVec<T> {
    data: Vec<T>,
}

impl<T> QueueVec<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn enqueue(&mut self, value: T) {
        self.data.push(value);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        Some(self.data.remove(0))
    }

    fn peek(&self) -> Option<&T> {
        self.data.first()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());

    stack.push(40);
    stack.push(50);

    println!("Peeking: ");
    println!("{:?}", stack.peek());
}
