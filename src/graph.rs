// reference to https://www.forcia.com/blog/001409.html

use num_traits::{PrimInt, zero};

#[derive(Debug, Clone)]
pub struct AdjGraph<T: PrimInt> {
    adj_mat: Vec<Vec<Option<T>>>,
    next: Vec<Vec<usize>>,
    size: usize,
}

impl<T: PrimInt> AdjGraph<T> {
    pub fn new(adj_mat: Vec<Vec<Option<T>>>) -> Self {
        let n = adj_mat.len();
        let mut next: Vec<Vec<usize>> = vec![];

        for _ in 0..n {
            next.push((0..n).map(|j| j).collect())
        }

        Self {
            adj_mat: adj_mat,
            next: next,
            size: n,
        } 
    }
    pub fn warshal_floyd(&mut self) {
        for k in 0..self.size {
            for i in 0..self.size {
                for j in 0..self.size {
                    if let (Some(dik), Some(dkj)) = (self.adj_mat[i][k], self.adj_mat[k][j]) {
                        if self.adj_mat[i][j].is_none() || self.adj_mat[i][j].unwrap() > dik + dkj {
                            self.adj_mat[i][j] = Some(dik + dkj);
                            self.next[i][j] = self.next[i][k]
                        }
                    }
                } 
            } 
        }
    }

    pub fn shortest_path(&self, start: usize, goal: usize) -> Option<(T, Vec<usize>)> {
        if self.adj_mat[start][goal].is_none() {
            return None;
        }

        let mut path = vec![start];
        let mut node = start;

        while node != goal {
            path.push(self.next[node][goal]);
            node = self.next[node][goal];
        }
        Some((self.adj_mat[start][goal].unwrap(), path))
    }
}

#[cfg(test)]
mod test {
    use super::AdjGraph;
    #[test]
    fn test_warshall_floyd() {
        let adj_mat = vec![
            vec![None, Some(1), Some(10), None, None],
            vec![None, None, None, Some(2), None],
            vec![None, Some(1), None, Some(3), Some(1)],
            vec![Some(7), None, None, None, Some(2)],
            vec![None, None, None, None, None],
        ];

        let mut graph = AdjGraph::new(adj_mat);
        graph.warshal_floyd();

        assert_eq!(graph.shortest_path(0, 1), Some((1, vec![0, 1])));
        assert_eq!(graph.shortest_path(0, 3), Some((3, vec![0, 1, 3])));
        assert_eq!(graph.shortest_path(3, 0), Some((7, vec![3, 0])));
        assert_eq!(graph.shortest_path(0, 4), Some((5, vec![0, 1, 3, 4])));
        assert_eq!(graph.shortest_path(4, 0), None);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Edge<T: PrimInt> {
    node: T,
    cost: T,
}

// for BinaryHeap
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State<T: PrimInt> {
    cost: T,
    position: usize,
    pre_node: usize,
}

impl<T: PrimInt> Ord for State<T> {
    fn cmp(&self, other: &State<T>) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl<T: PrimInt> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &State<T>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct ListGraph<T: PrimInt> {
    list_graph: Vec<Vec<Edge<T>>>,
    dist: Vec<usize>,
    pre_nodes: Vec<usize>,
    heap: std::collections::BinaryHeap<State<T>>,
    size: usize,
}

impl<T: PrimInt> ListGraph<T> {
    pub fn new(list_graph: Vec<Vec<Edge<T>>>) -> Self {
        let size = list_graph.len();
        let dist = (0..size).map(|_| std::usize::MAX).collect();
        let pre_nodes = (0..size).map(|i| i).collect();
        let heap: std::collections::BinaryHeap<State<T>> = std::collections::BinaryHeap::new();
        Self { list_graph, dist, pre_nodes, heap, size } }
        
    fn refresh(&mut self) {
        self.dist = (0..self.size).map(|_| std::usize::MAX).collect();
        self.pre_nodes = (0..self.size).map(|i| i).collect();
        self.heap = std::collections::BinaryHeap::new();
    }
}


impl<T: PrimInt> ListGraph<T> {
    fn _dijkstra(&mut self, start: usize, goal: usize) -> Option<(T, Vec<usize>)> {
        self.dist[start] = 0;
        self.heap.push(
            State {
                cost: zero(),
                position: start,
                pre_node: 0,
            }
        );

        while let Some(State {
            cost, position, pre_node,
        }) = self.heap.pop() {
            if cost.to_usize().unwrap() > self.dist[position] { continue; }
            self.pre_nodes[position] = pre_node;

            if position == goal {
                let mut v = goal;
                let mut path = vec![goal];
                while v != start {
                    path.push(self.pre_nodes[v]);
                    v = self.pre_nodes[v];
                }
                path.reverse();
                return Some((cost, path));
            }

            for edge in self.list_graph[position].iter() {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node.to_usize().unwrap(),
                    pre_node: position,
                };

                if next.cost.to_usize().unwrap() < self.dist[next.position] {
                    self.heap.push(next);
                    self.dist[next.position] = next.cost.to_usize().unwrap();
                }
            }
        }
        None
    }

    pub fn dijkstra(&mut self, start: usize, goal: usize) -> Option<(T, Vec<usize>)> {
        let ret = self._dijkstra(start, goal);
        self.refresh();
        ret
    }
}


#[cfg(test)]
mod test_list_graph {
    use super::{Edge, ListGraph};
    #[test]
    fn test_dijktstra_1() {
        let list_graph = vec![
            vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
            vec![Edge { node: 3, cost: 2 }],
            vec![
                Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 },
            ],
            vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
            vec![],
        ];

        let mut graph = ListGraph::new(list_graph);
        assert_eq!(graph.dijkstra(0, 1), Some((1, vec![0, 1])));
        assert_eq!(graph.dijkstra(0, 3), Some((3, vec![0, 1, 3])));
        assert_eq!(graph.dijkstra(3, 0), Some((7, vec![3, 0])));
        assert_eq!(graph.dijkstra(0, 4), Some((5, vec![0, 1, 3, 4])));
        assert_eq!(graph.dijkstra(4, 0), None);
    }
}


