/// graph: Tree
/// pos: Termination timing of dfs
#[derive(Debug, Clone)]
pub struct DfsTree {
    graph: Vec<Vec<usize>>,
    tree_index_to_dfs_index: Vec<usize>,
    dfs_index_to_tree_index: Vec<usize>,
    pos: Vec<usize>,
    cnt: usize,
}

impl DfsTree {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            tree_index_to_dfs_index: vec![0; n],
            dfs_index_to_tree_index: vec![0; n],
            pos: vec![0; n],
            cnt: 0,
        }
    }

    /// root: root of dfs_tree (tree_index)
    /// example
    /// ```
    /// use competitive::data_structures::dfs_tree::DfsTree;
    /// ```
    pub fn build(&mut self, root: usize) {
        let mut seen = vec![false; self.graph.len()];
        self.dfs(root, &self.graph.clone(), &mut seen);
    }

    /// v: original index
    /// graph: graph
    /// seen: whether already go or not
    fn dfs(&mut self, v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
        seen[v] = true;
        let dfs_ord = self.cnt;
        self.tree_index_to_dfs_index[v] = dfs_ord;
        self.dfs_index_to_tree_index[dfs_ord] = v;
        self.cnt += 1;

        for &next in graph[v].iter() {
            if seen[next] {
                continue;
            }
            self.dfs(next, graph, seen);
        }

        // 底まで見た
        self.pos[dfs_ord] = self.cnt;
    }

    /// subtree range
    /// return [v, pos[v])
    pub fn subtree_range(&self, dfs_index: usize) -> (usize, usize) {
        (dfs_index, self.pos[dfs_index])
    }

    pub fn dfs_index(&self, tree_index: usize) -> usize {
        self.tree_index_to_dfs_index[tree_index]
    }

    pub fn tree_index(&self, dfs_index: usize) -> usize {
        self.dfs_index_to_tree_index[dfs_index]
    }

    pub fn pos(&self, dfs_index: usize) -> usize {
        self.pos[dfs_index]
    }
}

#[cfg(test)]
mod test {
    use super::DfsTree;
    use crate::scanner::IO;
    #[test]
    fn test_build() {
        let input = br"
        7
        2 1
        2 3
        4 2
        4 5
        6 1
        3 7";
        let mut sc = IO::new(&input[..], Vec::new());

        let n: usize = sc.read();
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for _ in 0..n - 1 {
            let a: usize = sc.read();
            let b: usize = sc.read();
            graph[a - 1].push(b - 1);
            graph[b - 1].push(a - 1);
        }

        let mut dfs_tree = DfsTree::new(graph);
        dfs_tree.build(0);

        assert_eq!(dfs_tree.tree_index_to_dfs_index, vec![0, 1, 2, 4, 5, 6, 3]);

        assert_eq!(dfs_tree.pos, vec![7, 6, 4, 4, 6, 6, 7])
    }
}
