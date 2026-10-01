#![allow(unused)]

use std::collections::VecDeque;

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Self {
            adj: (0..vertices).map(|_| Vec::new()).collect(),
        }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.adj[a].push(b);
        self.adj[b].push(a);
    }

    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.adj.len()];
        let mut result = Vec::new();

        queue.push_back(start);
        visited[start] = true;

        while let Some(node) = queue.pop_front() {
            result.push(node);

            for &adj_node in &self.adj[node] {
                if !visited[adj_node] {
                    visited[adj_node] = true;
                    queue.push_back(adj_node);
                }
            }
            // `&` normally creates a reference. Then how come &adj_node has the type usize(as shown)?
            // This is a pattern: for &neighbor in ...
            // it says: "Each item Im receiving is a reference. Match that reference and give me the value inside it as neighbor"
            // So conceptually: item -> &usize -> dereference through pattern & -> usize -> neighbor
        }

        result
    }

    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut visited = vec![false; self.adj.len()];
        let mut result = Vec::new();

        stack.push(start);
        visited[start] = true;

        while let Some(node) = stack.pop() {
            result.push(node);

            for &adj_node in &self.adj[node] {
                if !visited[adj_node] {
                    visited[adj_node] = true;
                    stack.push(adj_node);
                }
            }
        }

        result
    }

    fn has_cycle(&self) -> bool {
        let mut stack: Vec<(usize, Option<usize>)> = Vec::new();
        let mut visited = vec![false; self.adj.len()];

        for start in 0..self.adj.len() {
            if !visited[start] {
                // (node, parent); starting from node 'start'
                stack.push((start, None));
                visited[start] = true;

                while let Some(node) = stack.pop() {
                    for &adj_node in &self.adj[node.0] {
                        if visited[adj_node] && node.1 != Some(adj_node) {
                            return true;
                        }

                        if !visited[adj_node] {
                            visited[adj_node] = true;
                            stack.push((adj_node, Some(node.0)));
                        }
                    }
                }
            }
        }

        false
    }
}

fn main() {}
