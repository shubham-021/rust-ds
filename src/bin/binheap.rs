#![allow(unused)]

#[derive(Debug)]
struct MaxHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    // fn swap(first: i32, second: i32) {
    //     let temp
    // }

    fn fix_heap(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent = (index - 1) / 2;
        if self.data[parent] >= self.data[index] {
            return;
        }

        self.data.swap(parent, index);
        self.fix_heap(parent);
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        let index = self.data.len() - 1;
        self.fix_heap(index);
    }

    fn heapify(&mut self, index: usize, size: usize) {
        let left = (2 * index) + 1;
        let right = (2 * index) + 2;
        let mut largest = index;

        if left < size && self.data[left] > self.data[largest] {
            largest = left;
        }

        if right < size && self.data[right] > self.data[largest] {
            largest = right;
        }

        if largest == index {
            return;
        }

        self.data.swap(index, largest);
        self.heapify(largest, size);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let size = self.data.len();
        self.data.swap(0, size - 1);

        let max = self.data.pop().unwrap();
        self.heapify(0, size - 1);

        Some(max)
    }

    fn peek(&self) -> Option<&T> {
        if self.data.is_empty() {
            return None;
        }

        return Some(&self.data[0]);
    }

    fn from_vec(data: Vec<T>) -> Self {
        let size = data.len();

        let mut heap = Self { data };

        for i in (0..size / 2).rev() {
            heap.heapify(i, size);
        }

        heap
    }
}

fn main() {
    let mut maxhe = MaxHeap::new();

    maxhe.push(50);
    maxhe.push(30);
    maxhe.push(40);
    maxhe.push(60);
    maxhe.push(20);
    maxhe.push(80);

    println!("{:?}", maxhe);
    println!("Max value: {:?}", maxhe.peek().unwrap());
    // println!("Max value: {:?}", *maxhe.peek().unwrap()); both prints the same thing
}
