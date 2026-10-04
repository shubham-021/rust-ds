#![allow(unused)]

// https://www.youtube.com/watch?v=aBxjDBC4M1U (striver's dsu video)

/*  DSU (Disjoint Set Union)
 *  It is used when we have a collection of elements divided into groups(sets) and we need to effeciently:
 *      - Find which group an element belongs to
 *      - Union two groups together
 *
 *  For example: Initially: {1} {2} {3} {4} {5}
 *               if we do: union(1,2) and union(3,4)
 *               we get:  {1,2} {3,4} {5}
 *
 *               Then: union(2,3) => {1,2,3,4} {5}
 *               And we ask: find(1), find(4) to determine they're in the same set
 *
 *  The key idea is that each set is represented as a tree, and every node ultimately points to a representative/root
 *
 *  Use case examples:
 *      - Network connectivity:
 *          Suppose you have computer: 1 2 3 4 5, Initially every computer is separate {1} {2} {3} {4} {5}
 *          Now cables are connected: connect(1,2) connect(2,3) connect(4,5)
 *          DSU represents: {1,2,3} {4,5}
 *          Now you can quickly ask: areConnected(1,3) -> yes, areConnected(1,5) -> no
 *
 *          Then if a new cable connects 3 and 5: union(3, 5), Now -> {1,2,3,4,5} and areConnected(1,5) -> yes
 *
 * Where this pattern appears:
 *      - Network connectivity: Are two computers/ servers in the same connected network ?
 *      - Graph problems: Give edge one by one, determine whether two vertices belongs to same connected component
 *      - Detecting cycles in an undirected graph: if A and B are already in the same set, adding this edge creates a cycle
 *      - Krushkal's Minimum spanning tree
 */

/* Normal Union:
struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.find(self.parent[x]) <-- not doing path compression
        self.parent[x] = self.find(self.parent[x]) <--- path compression (while backtracking change the parent of every node to its ultimate parent)(make self a mut ref)
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a != root_b {
            self.parent[root_b] = root_a;
        }
    }
}
*/

/* Union by size
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![0; n],
        }
    }

    fn find(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.find(self.parent[x])
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        let (parent_root, child_root) = if self.size[root_a] > self.size[root_b] {
            (root_a, root_b)
        } else {
            (root_b, root_a)
        };

        self.parent[child_root] = parent_root;
        self.size[parent_root] += self.size[child_root];
    }
}
*/

/* Union by rank */
struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.parent[x] = self.find(self.parent[x]);
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.rank[root_a] > self.rank[root_b] {
            self.parent[root_b] = root_a;
        } else if self.rank[root_a] < self.rank[root_b] {
            self.parent[root_a] = root_b;
        } else {
            self.parent[root_a] = root_b;
            self.rank[root_b] += 1;
        }
    }
}

// Note: Rank is not necessarily the actual height after path compression.

fn main() {}
