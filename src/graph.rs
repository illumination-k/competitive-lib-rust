use num_traits::{PrimInt};

#[derive(Debug)]
pub struct AdjGraph<T: PrimInt> {
    adj_mat: Vec<Vec<Option<T>>>,
    next: Vec<Vec<usize>>,
    size: usize,
}

impl<T: PrimInt> AdjGraph<T> {
    pub fn new(adj_mat: Vec<Vec<Option<T>>>) -> Self {
        let n = adj_mat.len();
        let mut next: Vec<Vec<usize>> = vec![];

        for _i in 0..n {
            next.push((0..n).map(|j| j).collect())
        }

        Self {
            adj_mat: adj_mat,
            next: next,
            size: n,
        } 
    }
}

impl<T: PrimInt> AdjGraph<T> {
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