#![allow(unused)]

struct DirectedGraph {
    adj: Vec<Vec<isize>>, // (weight, node)
}

impl DirectedGraph {
    fn new(vertices: usize) -> Self {
        Self {
            adj: vec![vec![isize::MAX; vertices]; vertices], // isize::MAX -> unreachable node
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: isize) {
        self.adj[from][to] = weight;
    }

    fn floyd_warshall(&self) -> Option<Vec<Vec<isize>>> {
        let vertices = self.adj.len();
        let mut result = self.adj.clone();

        for i in 0..vertices {
            result[i][i] = 0;
        }

        for i in 0..vertices {
            for j in 0..vertices {
                for k in 0..vertices {
                    if result[j][i] == isize::MAX || result[i][k] == isize::MAX {
                        continue;
                    }

                    let via_i = result[j][i] + result[i][k];
                    if via_i < result[j][k] {
                        result[j][k] = via_i;
                    }
                }
            }

            let mut contains_negative = false;
            for i in 0..vertices {
                if result[i][i] < 0 {
                    contains_negative = true;
                    break;
                }
            }

            if contains_negative {
                return None;
            }
        }

        //         let mut contains_negative = false;
        //         for i in 0..vertices {
        //             if result[i][i] < 0 {
        //                 contains_negative = true;
        //                 break;
        //             }
        //         }
        //
        //         if contains_negative {
        //             return None;
        //         }

        Some(result)
    }
}

fn main() {}
