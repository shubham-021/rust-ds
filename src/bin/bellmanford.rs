#![allow(unused)]

/*  Dijkstra: Shortest path from one source to every other vertex, when edge weights are non negative
*
*   Bellman-Ford solves the same problem, but it can also handle negative edge weights
*/

struct DirectedGraph {
    adj: Vec<Vec<(i32, usize)>>, // (weight, node)
}

impl DirectedGraph {
    fn new(vertices: usize) -> Self {
        Self {
            adj: (0..vertices).map(|_| Vec::new()).collect(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.adj[from].push((weight, to));
    }

    fn bellman_ford(&self) -> Vec<i32> {
        let vertices = self.adj.len();
        let mut dist = vec![i32::MAX; vertices];
        dist[0] = 0;

        for _ in 0..vertices - 1 {
            let mut flag = false;
            for i in 0..vertices {
                for &(weight, node) in &self.adj[i] {
                    if dist[i] != i32::MAX && dist[i] + weight < dist[node] {
                        dist[node] = dist[i] + weight;
                        flag = true;
                    }
                }
            }

            if !flag {
                break;
            }
        }

        dist
    }
}

/*  Bellman-ford is useful for negative weights in directed graphs, but an undirected graph containing a negative
*   weight edge automatically contains a negative cycle
*
*   Suppose: A --( -5 )--B
*   Because it's undirected, you actually have: A -- ( -5 ) --> B, A <-- ( -5 ) -- B
*   So you can do, A -> B -> A -> B -> ... with cost -5 + -5 + -5 + ...
*   This is a negative cycle.
*
*   Why negative cycle cause problem if it is sure that the algo will run only for V-1 iteration:
*       After V-1 rounds, we don't necessarily know whether the distances are truly final if a negative cycle exists.
*       The reasons V-1 is enough when there is no negative cycle is: 'A shortes path can contain at most V-1 edges'
*       But with a negative cycle, there is no finite shorted path. You can keep going around the cycle and make the
*       cost smaller forever.
*/

fn main() {}
