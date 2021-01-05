/// graph: Tree
/// map: DfsTree index to Tree index
/// pos: Termination timing of dfs
#[derive(Debug, Clone)]
pub struct DfsTree {
    graph: Vec<Vec<usize>>,
    tree_index_to_dfs_index: Vec<usize>,
    dfs_index_to_tree_index: Vec<usize>,
    pos: Vec<usize>,
    cnt: usize
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
    pub fn build(&mut self, root: usize) {
        let mut seen = vec![false; self.graph.len()];
        self.dfs(root, &self.graph.clone(), &mut seen);
    }

    /// v: original index
    /// graph: graph
    /// seen: whether already go or not
    fn dfs(&mut self, v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
        if seen[v] { return; }

        seen[v] = true;
        let dfs_ord = self.cnt;
        self.tree_index_to_dfs_index[v] = dfs_ord;
        self.dfs_index_to_tree_index[dfs_ord] = v;
        self.cnt += 1;
        
        for i in 0..graph[v].len() {
            self.dfs(graph[v][i], graph, seen);
        }

        // 底まで見た
        self.pos[dfs_ord] = self.cnt;
    }

    /// subtree range
    /// [v, pos[v])
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