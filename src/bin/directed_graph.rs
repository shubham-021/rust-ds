#![allow(unused)]

use std::collections::VecDeque;

struct Graph {
    adj: Vec<Vec<usize>>,
    indegree: Vec<usize>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Self {
            adj: (0..vertices).map(|_| Vec::new()).collect(),
            indegree: vec![0; vertices],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adj[from].push(to);
        self.indegree[to] += 1;
    }

    // Kahn's Algorithm
    fn topological_sort(&mut self) -> Option<Vec<usize>> {
        let total_vertices = self.adj.len();
        let mut indegrees = self.indegree.clone();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        for i in 0..total_vertices {
            if indegrees[i] == 0 {
                queue.push_back(i);
            }
        }

        while let Some(index) = queue.pop_front() {
            result.push(index);

            for &neighbor in &self.adj[index] {
                indegrees[neighbor] -= 1;
                if indegrees[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        if result.len() < total_vertices {
            return None;
        }

        Some(result)
    }
}

fn main() {
    let mut ug = Graph::new(4);
    ug.add_edge(0, 1);
    ug.add_edge(1, 3);
    ug.add_edge(2, 3);
    ug.add_edge(0, 2);

    let topo_sort = ug.topological_sort();

    println!("Toposort of the graph is: {:?}", topo_sort);
}
