#![allow(unused)]
use std::collections::hash_map::DefaultHasher;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::mem::take;

// PartialEq means the type supports equality comparison, but it may have values whose equality behavior doesn't satisfy the
// stronger mathematical laws. Eq guarantees those laws hold for every value.

// PartialOrd means "This type supports ordering comparisons, but the ordering may not be complete for every pair of values."
// Ord means "This type has a complete ordering: every pair of values can be ordered."

const LOAD_FACTOR: f64 = 0.75;

#[derive(Debug)]
struct HashMap<K, V> {
    buckets: Vec<Vec<(usize, K, V)>>,
    size: usize,
}

// buckets: vec![Vec::new(); 16] -> means create the value once, then clone it n times
// so rust will need: Vec<(K,V)>: Clone, for Vec<(K,V)> to be Clone, (K,V) must be Clone, which means
// K: Clone
// V: Clone
//
// But we dont want to require our HashMap's keys and values to implement Clone just to create empty buckets
// Better solution: Use an iterator
//
// Why we need to call collect() after calling map(), what does map() returns ?
// map() does not return a Vec, It returns an iterator. For example:
// let x = (0..4).map(|_| Vec::new());
// (0..4) -> map() -> Iterator -> 4 Vec::new()
//
// But those Vec::new() values aren't al created immediately. The iterator produces them as you consume it.
// So x is essentially, Iterator<Item = Vec<_>>
//
// then, collect() takes that iterator and collects its produced elements into some collection.
// So, collect() is what actually gives us our outer Vec.
//
// How does Rust know what to collect into ?
// Because we told it: let buckets: Vec<Vec<K, V>> = ...

impl<K: Hash + Eq + Display, V> HashMap<K, V> {
    fn new() -> Self {
        Self {
            buckets: (0..16).map(|_| Vec::new()).collect(),
            size: 0,
        }
    }

    fn get_hash_and_bucket(&self, key: &K) -> (usize, usize) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash = hasher.finish() as usize;

        (hash % self.buckets.len(), hash)
    }

    fn check_load_factor(&self) -> bool {
        let buckets_list_size = self.buckets.len();
        (self.size as f64 / buckets_list_size as f64) >= LOAD_FACTOR
    }

    fn insert(&mut self, key: K, value: V) {
        let (bucket_index, hash) = self.get_hash_and_bucket(&key);

        for entry in self.buckets[bucket_index].iter_mut() {
            if (entry.1 == key) {
                entry.2 = value;
                return;
            }
        }

        self.buckets[bucket_index].push((hash, key, value));
        self.size += 1;

        if self.check_load_factor() {
            self.resize();
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let (bucket_index, _) = self.get_hash_and_bucket(key);

        // for entry in self.buckets[bucket_index].iter() {
        //     if &entry.1 == key {
        //         return Some(&entry.2);
        //     }
        // }

        for entry in &self.buckets[bucket_index] {
            if &entry.1 == key {
                return Some(&entry.2);
            }
        }

        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let (bucket_index, _) = self.get_hash_and_bucket(key);

        let size = self.buckets[bucket_index].len();
        for i in 0..size {
            if &self.buckets[bucket_index][i].1 == key {
                // let (k, v) = self.buckets[bucket_index].remove(i);
                // Vec::remove(index) shifts every element after i one position to the left, that's O(n) within the bucket
                let (_, _, v) = self.buckets[bucket_index].swap_remove(i);
                self.size -= 1;
                // swap_remove(index) is O(1), but changes the ordering, since order doesn't matter here, so swap_remove is perfect for this
                return Some(v);
            }
        }

        return None;
    }

    fn resize(&mut self) {
        let old_size = self.buckets.len();
        let new_size = old_size * 2;

        let old_buckets = take(&mut self.buckets);
        self.buckets = (0..new_size).map(|_| Vec::new()).collect();
        // self.size = 0;
        // or
        // let new_buckets = (0..old_size * 2).map(|_| Vec::new()).collect();
        // let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);

        for bucket in old_buckets {
            for (hash, k, v) in bucket {
                let new_bucket_index = hash % new_size;
                self.buckets[new_bucket_index].push((hash, k, v));
                // self.size += 1;
            }
        }
    }
}

fn main() {
    let mut hash_m = HashMap::new();
    hash_m.insert("a", 1);
    hash_m.insert("b", 2);
    hash_m.insert("c", 3);
    hash_m.insert("d", 4);
    hash_m.insert("e", 5);
    hash_m.insert("f", 6);
    hash_m.insert("g", 7);
    hash_m.insert("h", 8);
    hash_m.insert("i", 9);
    hash_m.insert("j", 10);
    hash_m.insert("k", 11);

    println!(
        "\n\n HashMap produced for 11 insertion, hashmap has {} element, hashmap size is: {} : {:?}\n\n",
        hash_m.size,
        hash_m.buckets.len(),
        hash_m
    );

    hash_m.insert("l", 12);
    hash_m.insert("m", 13);
    hash_m.insert("n", 14);

    println!(
        "\n\n HashMap produced for 3 more insertion, hashmap has {} element, hashmap size is: {} :  {:?}\n\n",
        hash_m.size,
        hash_m.buckets.len(),
        hash_m
    );

    println!(
        "\n\nRemoved the value associated with the key \"a\" -> value: {:?}, current hashmap with number of elements: {} and size {} is: {:?}",
        hash_m.remove(&"a"),
        hash_m.size,
        hash_m.buckets.len(),
        hash_m
    );

    println!(
        "\n\nRemoved the value associated with the key \"g\" -> value: {:?}, current hashmap with number of elements: {} and size {} is: {:?}",
        hash_m.remove(&"g"),
        hash_m.size,
        hash_m.buckets.len(),
        hash_m
    );

    println!(
        "\n\nRemoved the value associated with the key \"n\" -> value: {:?}, current hashmap with number of elements: {} and size {} is: {:?}",
        hash_m.remove(&"n"),
        hash_m.size,
        hash_m.buckets.len(),
        hash_m
    );

    println!(
        "\n\nValue associated with the key \"b\" is {:?}",
        hash_m.get(&"b")
    );

    println!(
        "\n\nValue associated with the key \"i\" is {:?}",
        hash_m.get(&"i")
    );

    println!(
        "\n\nValue associated with the key \"l\" is {:?}",
        hash_m.get(&"l")
    );
}
