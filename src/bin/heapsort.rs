#![allow(unused)]

fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify_rec(arr, n, i);
    }

    for i in (0..n) {
        arr.swap(0, n - i - 1);
        heapify_rec(arr, n - i - 1, 0);
    }
}

fn heapify_rec<T: Ord>(arr: &mut [T], heap_size: usize, index: usize) {
    let left = index * 2 + 1;
    let right = index * 2 + 2;
    let mut largest = index;

    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }

    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != index {
        arr.swap(index, largest);
        heapify_rec(arr, heap_size, largest);
    }
}

fn heapify_iter<T: Ord>(arr: &mut [T], heap_size: usize, index: usize) {
    let mut index = index;

    loop {
        let left = index * 2 + 1;
        let right = index * 2 + 2;
        let mut largest = index;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }

        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest == index {
            break;
        }

        arr.swap(largest, index);
        index = largest;
    }
}

fn main() {
    let mut arr = [7, 1, 3, 6, 5, 4, 2];
    heap_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

/*
 *  One heapify call [heapify(arr, n, index)]:
 *      Starting from one node, it can move down only on path. The maximum path length is the height of the heap: log(n)
 *      So, One heapify = O(log(n))
 *
 *  Then heap sort calls heapify() approximately n times, therefore heap sort is O(nlog(n))
 *
 *  for i in (0..n/2).rev(){
 *      heapify(...);
 *  }
 *
 *  there are about n/2 calls, but not every heapify costs log(n). Most nodes are near the bottom and can only move 1-2 levels
 *  therefore: build heap -> O(n)
 *
 *  recursive heapify -> O(log(n)) <- call stack
 *  iterative heapify -> O(1)
 *
 *  same time complexity for both
 */
