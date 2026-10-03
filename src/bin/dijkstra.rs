#![allow(unused)]

#[derive(Debug)]
struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn fix_heap(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent = (index - 1) / 2;
        if self.data[parent] <= self.data[index] {
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
        let mut smallest = index;

        if left < size && self.data[left] < self.data[smallest] {
            smallest = left;
        }

        if right < size && self.data[right] < self.data[smallest] {
            smallest = right;
        }

        if smallest == index {
            return;
        }

        self.data.swap(index, smallest);
        self.heapify(smallest, size);
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

    fn from_vec(data: Vec<T>) -> Self {
        let size = data.len();

        let mut heap = Self { data };

        for i in (0..size / 2).rev() {
            heap.heapify(i, size);
        }

        heap
    }
}

#[derive(Debug)]
struct WeightedDirectedGraph {
    adj: Vec<Vec<(usize, usize)>>,
}

struct WeightedUndirectedGraph {
    adj: Vec<Vec<(usize, usize)>>,
}

impl WeightedDirectedGraph {
    fn new(vertices: usize) -> Self {
        Self {
            adj: (0..vertices).map(|_| Vec::new()).collect(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.adj[from].push((to, weight));
    }

    fn dijkstra(&self, start: usize) -> Vec<usize> {
        let mut dist = vec![usize::MAX; self.adj.len()];

        let mut min_heap = MinHeap::new();

        // for i in (0..self.adj[start].len()) {
        //     let (node, weight) = self.adj[start][i];
        //     min_heap.push((weight, node));
        // }

        min_heap.push((0, start));

        while let Some((weight, node)) = min_heap.pop() {
            if weight < dist[node] {
                dist[node] = weight;

                for &(n_node, n_weight) in &self.adj[node] {
                    let new_distance = weight + n_weight;
                    if new_distance < dist[n_node] {
                        min_heap.push((new_distance, n_node));
                    }
                }
            }
        }

        dist
    }
}

impl WeightedUndirectedGraph {
    fn new(vertices: usize) -> Self {
        Self {
            adj: (0..vertices).map(|_| Vec::new()).collect(),
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, weight: usize) {
        self.adj[a].push((b, weight));
        self.adj[b].push((a, weight));
    }

    fn dijkstra(&self, start: usize) -> Vec<usize> {
        let mut dist = vec![usize::MAX; self.adj.len()];

        let mut min_heap = MinHeap::new();

        min_heap.push((0, start));

        while let Some((weight, node)) = min_heap.pop() {
            if weight < dist[node] {
                dist[node] = weight;

                for &(n_node, n_weight) in &self.adj[node] {
                    let new_distance = weight + n_weight;
                    if new_distance < dist[n_node] {
                        min_heap.push((new_distance, n_node));
                    }
                }
            }
        }

        dist
    }
}

fn main() {
    let mut directed_graph = WeightedDirectedGraph::new(4);
    directed_graph.add_edge(0, 1, 4);
    directed_graph.add_edge(0, 2, 1);
    directed_graph.add_edge(1, 3, 2);
    directed_graph.add_edge(2, 3, 5);

    println!(
        "Shortes path to all node from node 0 is: {:?}",
        directed_graph.dijkstra(0)
    );

    let mut undirected_graph = WeightedUndirectedGraph::new(4);
    undirected_graph.add_edge(0, 1, 4);
    undirected_graph.add_edge(0, 2, 1);
    undirected_graph.add_edge(1, 3, 2);
    undirected_graph.add_edge(2, 3, 5);

    println!(
        "\n\nShortes path to all node from node 0 is: {:?}",
        undirected_graph.dijkstra(0)
    );
}
