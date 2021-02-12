pub mod listgraph {
    use num_traits::{Bounded, NumCast, One, Zero};
    use std::{collections::{BinaryHeap, HashSet, VecDeque}, fmt, ops::{Index, IndexMut}, writeln};
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Edge<W> {
        target: usize,
        weight: W,
    }

    impl<W> Edge<W> 
        where W: One + Copy
    {
        pub fn new(target: usize, weight: W) -> Self {
            Self { target, weight }
        }

        pub fn new_unweighted(target: usize) -> Self {
            Self { target, weight: W::one() }
        }

        pub fn target(&self) -> usize {
            self.target
        }

        pub fn weight(&self) -> W {
            self.weight
        }
    }

    impl<W> ListGraph<W>
        where W: ToString + One + PartialEq + Clone + Copy
    {   
        /// create dot format for graphviz
        /// ```rust
        /// use competitive::graph::listgraph::*;
        /// let vec = vec![
        ///     (0, 1, 1),
        ///     (0, 2, 4),
        ///     (2, 0, 1),
        ///     (1, 2, 2),
        ///     (3, 1, 1),
        ///     (3, 2, 5),
        /// ];
        /// let graph: ListGraph<isize> = ListGraph::weighted_from(vec, 0, Direction::DiGraph);
        /// let dot = graph.to_dot(&Direction::DiGraph);
        /// assert_eq!(
        ///     dot,
        ///     vec![
        ///         "digraph example {",
        ///         "  0 -> 1",
        ///         "  0 -> 2 [ label = 4 ]",
        ///         "  1 -> 2 [ label = 2 ]",
        ///         "  2 -> 0",
        ///         "  3 -> 1",
        ///         "  3 -> 2 [ label = 5 ]",
        ///         "}"
        ///     ].iter().map(|x| x.to_string()).collect::<Vec<String>>()
        /// )
        /// ```
        pub fn to_dot(&self, graph_type: &Direction) -> Vec<String> {
            fn make_dot_edge<W>(source: usize, target: usize, weight: W, graph_type: &Direction) -> String
                where W: One + ToString + PartialEq
            {
                let arr = match graph_type {
                    Direction::DiGraph => { "->" },
                    Direction::UnGraph => { "-" }
                };

                if weight == W::one() {
                    format!("  {} {} {}", source, arr, target)
                } else {
                    format!("  {} {} {} [ label = {} ]", source, arr, target, weight.to_string())
                }
            }

            let mut seen_edge = HashSet::new();
            let mut dot = match graph_type {
                Direction::DiGraph => {
                    vec!["digraph example {".to_string()]
                },
                Direction::UnGraph => {
                    vec!["graph example {".to_string()]
                }
            };
            for source in 0..self.len() {
                for e in self.neighbors(source) {
                    let (target, weight) = (e.target(), e.weight());
                    let dot_edge = make_dot_edge(source, target, weight, &graph_type);
                    if seen_edge.contains(&dot_edge) { continue; }
                    dot.push(dot_edge.clone());
                    seen_edge.insert(dot_edge);
                }
            }

            dot.push("}".to_string());
            dot
        } 
    }

    impl<W> PartialOrd for Edge<W>
        where W: PartialEq + PartialOrd
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.weight
                .partial_cmp(&other.weight)
        }
    }

    impl<W> Eq for Edge<W> where W: PartialEq + PartialOrd {}

    impl<W> Ord for Edge<W>
        where W: PartialOrd + PartialEq + PartialOrd
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.weight.partial_cmp(&self.weight).expect("No Nan").then_with(|| self.target.cmp(&other.target))
        }
    }

    #[derive(Clone)]
    pub struct ListGraph<W> {
        graph: Vec<Vec<Edge<W>>>
    }

    impl<W> fmt::Debug for ListGraph<W>
        where W: Copy + One + Clone + ToString
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "")?;
            for source in 0..self.len() {
                let targets = &self[source];
                let mut write_str = String::new();
                write_str += &(source.to_string() + " -> ");
                write_str += &targets.iter().map(|x| x.target().to_string()).collect::<Vec<String>>().join(" ");
                writeln!(f, "{}", write_str)?
            }
            Ok(())        
        }
    }

    #[derive(Debug, Clone)]
    pub enum Direction {
        DiGraph,
        UnGraph
    }

    fn add_target<W: One + Copy>(
        source: usize,
        target: usize,
        weight: W,
        graph_type: &Direction,
        v: &mut Vec<Vec<Edge<W>>>
    ) {
        match graph_type {
            Direction::DiGraph => {
                v[source].push(Edge::new(target, weight))
            },
            Direction::UnGraph => {
                v[source].push(Edge::new(target, weight));
                v[target].push(Edge::new(source, weight))
            }
        }
    }

    impl<W> ListGraph<W>
        where W: Clone + One + Copy
    {
        pub fn new(n: usize) -> Self {
            Self {
                graph: vec![vec![]; n]
            }
        }

        /// create unweighted ListGraph.  
        /// offset: index = val - offset  
        /// graph_type: Undirect or Direct  
        pub fn unweighted_from(vec: Vec<(usize, usize)>, offset: usize, graph_type: Direction) -> Self {
            let max_val = *vec.iter().map(|(i, j)| std::cmp::max(i, j)).max().unwrap() + 1 - offset;
            let mut graph = vec![vec![]; max_val];
            for &(a, b) in vec.iter() {
                add_target(a-offset, b-offset, W::one(), &graph_type, &mut graph)
            }

            Self {
                graph
            }
        }

        /// create weighted ListGraph<W>.
        /// offset: index = val - offset
        /// graph_type: Undirect or Direct
        pub fn weighted_from(vec: Vec<(usize, usize, W)>, offset: usize, graph_type: Direction) -> Self {
            let max_val = *vec.iter().map(|(i, j, _w)| std::cmp::max(i, j)).max().unwrap() + 1 - offset;
            let mut graph = vec![vec![]; max_val];
            for &(a, b, w) in vec.iter() {
                add_target(a-offset, b-offset, w, &graph_type, &mut graph)
            }

            Self {
                graph
            }
        }

        /// Reverse Direction of Graph
        pub fn t(&self) -> Self {
            let mut vec = vec![];
            for source in 0..self.len() {
                for e in self.neighbors(source) {
                    vec.push((e.target(), source, e.weight()))
                }
            }

            ListGraph::<W>::weighted_from(vec, 0, Direction::DiGraph)
        }

        pub fn len(&self) -> usize {
            self.graph.len()
        }
    }

    impl<W> Index<usize> for ListGraph<W> {
        type Output = Vec<Edge<W>>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.graph[index]
        }
    }

    impl<W> IndexMut<usize> for ListGraph<W> {
        fn index_mut(&mut self, index: usize) -> &mut Vec<Edge<W>> {
            &mut self.graph[index]
        }
    }

    pub type UnweightedListGraph = ListGraph<usize>;

    pub struct NeighborhoodIter<'a, W> {
        source: usize,
        counter: usize,
        graph: &'a ListGraph<W>,
    }

    pub struct NeighborhoodUnweightedIter<'a, W> {
        source: usize,
        counter: usize,
        graph: &'a ListGraph<W>,
    }


    impl<'a, W> Iterator for NeighborhoodIter<'a, W> {
        type Item = &'a Edge<W>;

        fn next(&mut self) -> Option<Self::Item> {
            self.counter += 1;
            self.graph[self.source].get(self.counter - 1)
        }
    }

    impl <'a, W> Iterator for NeighborhoodUnweightedIter<'a, W>
        where W: Copy + One
    {
        type Item = &'a usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.counter >= self.graph[self.source].len() {
                return None
            }
            self.counter += 1;
            Some(&(self.graph[self.source][self.counter - 1].target))
        }
    }

    pub trait NeighborhoodExt<W> {
        /// get neighborhood of source with weight  
        fn neighbors(&self, source: usize) -> NeighborhoodIter<W>;

        /// get neighborhood of source without weight
        fn neighbors_unweighted(&self, source: usize) -> NeighborhoodUnweightedIter<W>;
    }

    impl<W> NeighborhoodExt<W> for ListGraph<W> {
        fn neighbors(&self, source: usize) -> NeighborhoodIter<W> {
            NeighborhoodIter {
                source,
                counter: 0,
                graph: self
            }
        }

        fn neighbors_unweighted(&self, source: usize) -> NeighborhoodUnweightedIter<W> {
            NeighborhoodUnweightedIter {
                source,
                counter: 0,
                graph: self
            }
        }
    }

    /// **Diktstra**   
    /// O(|E+V|log(|V|))   
    /// let E be edge number, let V be vertex number.
    /// return distance from start and prev nodes information.
    /// restore_path function can create shortest path from start to goal from prev nodes.
    /// ```rust
    /// use competitive::graph::listgraph::*;
    /// let vec = vec![
    ///     (0, 1, 1),
    ///     (0, 2, 4),
    ///     (2, 0, 1),
    ///     (1, 2, 2),
    ///     (3, 1, 1),
    ///     (3, 2, 5),
    /// ];
    /// let graph: ListGraph<isize> = ListGraph::weighted_from(vec, 0, Direction::DiGraph);
    /// 
    /// let (w, prev_nodes) = diktstra(&graph, 1);
    /// assert_eq!(w, vec![3, 0, 2, std::isize::MAX]);
    /// assert_eq!(restore_path(1, 0, &prev_nodes), vec![1, 2, 0]);
    /// ```
    pub fn diktstra<W>(graph: &ListGraph<W>, start: usize) -> (Vec<W>, Vec<usize>) 
        where W: Copy + One + Zero + PartialEq + PartialOrd + NumCast + Bounded + fmt::Debug
    {
        // initialize dist
        let mut dist: Vec<W> = vec![W::max_value(); graph.len()];
        dist[start] = W::zero();

        // initialize BinaryHeap
        let mut bq = BinaryHeap::new();
        // add dumy edge and start position.
        // edge weight must be smaller than zero()
        bq.push((Edge::new(0, W::zero()), start));

        // initialize prev nodes
        let mut prev_nodes = vec![std::usize::MAX; graph.len()];

        while let Some((edge, position)) = bq.pop() {
            if dist[position] < edge.weight() { continue; }
            for &e in graph.neighbors(position) {
                if dist[e.target()] <= dist[position] + e.weight() { continue; }
                bq.push((e, e.target()));
                dist[e.target()] = dist[position] + e.weight();
                prev_nodes[e.target()] = position;
            }
        }

        (dist, prev_nodes)
    }

    /// **BFS**  
    /// O(|V+E|)  
    /// let E be edge numbers, Let V be vertex numbers
    /// return distance from start and prev nodes information
    /// restore_path function can create shortest path from start to goal from prev nodes.
    /// ```rust
    /// use competitive::graph::listgraph::*;
    /// let vec = vec![
    ///     (1, 2),
    ///     (1, 4),
    ///     (2, 4),
    ///     (4, 3)    
    /// ];
    /// let graph: UnweightedListGraph = ListGraph::unweighted_from(vec, 1, Direction::DiGraph);
    /// let (dist, prev_nodes) = bfs(&graph, 0);
    /// assert_eq!(dist, vec![0, 1, 2, 1]);
    /// assert_eq!(restore_path(0, 2, &prev_nodes), vec![0, 3, 2]);
    /// ```
    pub fn bfs<W>(graph: &ListGraph<W>, start: usize) -> (Vec<isize>, Vec<usize>)
        where W: Copy + One + Clone
    {
        let mut dist = vec![-1; graph.len()];
        dist[start] = 0;
        let mut prev_nodes = vec![std::usize::MAX; graph.len()];

        let mut vq = VecDeque::new();
        vq.push_back(start);

        while let Some(position) = vq.pop_front() {
            for &next in graph.neighbors_unweighted(position) {
                if dist[next] != -1 { continue; }
                dist[next] = dist[position] + 1;
                prev_nodes[next] = position;
                vq.push_back(next)
            }
        }

        (dist, prev_nodes)
    }

    /// restore shortest path from start to goal
    pub fn restore_path(start: usize, goal: usize, prev_nodes: &Vec<usize>) -> Vec<usize> {
        let mut res = vec![];
        let mut pos = goal;
        while pos != start {
            res.push(pos);
            pos = prev_nodes[pos];
        }
        res.push(start);
        res.reverse();
        res
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DfsResultType {
        FirstAndLastOrd,
        TimeStamp,
        NoOrd,
    }

    #[derive(Debug, Clone)]
    pub struct DfsResults {
        pub seen: Vec<bool>,
        pub first_order: Vec<usize>,
        pub last_order: Vec<usize>,
        pub result_type: DfsResultType,
        ptrs: Vec<usize>,
    }

    impl DfsResults {
        pub fn new(size: usize, result_type: DfsResultType) -> Self {
            let ptrs = match &result_type {
                DfsResultType::FirstAndLastOrd => { vec![0; 2] },
                DfsResultType::TimeStamp => { vec![0; 1] },
                DfsResultType::NoOrd => { vec![] }
            };
            Self {
                seen: vec![false; size],
                first_order: vec![0; size],
                last_order: vec![0; size],
                result_type: result_type,
                ptrs: ptrs,
            }
        }

        /// update first ord  
        pub fn update_first_order(&mut self, pos: usize) {
            match self.result_type {
                DfsResultType::FirstAndLastOrd => {
                    self.first_order[pos] = self.ptrs[0];
                    self.ptrs[0] += 1;
                },
                DfsResultType::TimeStamp => {
                    self.first_order[pos] = self.ptrs[0];
                    self.ptrs[0] += 1;
                },
                DfsResultType::NoOrd => {}
            }
        }

        /// update last ord  
        pub fn update_last_order(&mut self, pos: usize) {
            match self.result_type {
                DfsResultType::FirstAndLastOrd => {
                    self.last_order[pos] = self.ptrs[1];
                    self.ptrs[1] += 1;
                },
                DfsResultType::TimeStamp => {
                    self.last_order[pos] = self.ptrs[0];
                    self.ptrs[0] += 1;
                },
                DfsResultType::NoOrd => {}
            }
        }
    }

    /// **DFS**  
    /// O(|V+E|)  
    /// let E be edge numbers, Let V be vertex numbers.
    /// return the DfsResults struct, which has seen results and first and last ord or time stamps.
    pub fn dfs<W>(start: usize, graph: &ListGraph<W>, result_type: DfsResultType) -> DfsResults
        where W: Copy + Clone + One
    {   
        /// internal dfs implementation
        fn _dfs<W>(start: usize, graph: &ListGraph<W>, dfs_result: &mut DfsResults)
            where W: Copy + Clone + One
        {
            // memo of first ord
            dfs_result.update_first_order(start);

            dfs_result.seen[start] = true;
            for &next in graph.neighbors_unweighted(start) {
                if dfs_result.seen[next] { continue; }
                _dfs(next, graph, dfs_result)
            }

            // memo of last ord
            dfs_result.update_last_order(start);
        }

        // initialize dfs results
        let mut dfs_result = DfsResults::new(graph.len(), result_type);
        dfs_result.seen[start] = true;

        // rec dfs
        _dfs(start, graph, &mut dfs_result);
        dfs_result
    }
}

#[cfg(test)]
mod test {
    use super::listgraph::*;
    #[test]
    fn test_fmt() {
        let graph: ListGraph<usize> = ListGraph::unweighted_from(vec![(0, 1), (1, 2), (4, 1), (1, 3)], 0, Direction::UnGraph);
        dbg!(&graph);
    }

    #[test]
    fn test_iter() {
        let graph: ListGraph<usize> = ListGraph::unweighted_from(vec![(0, 1), (1, 2), (4, 1), (1, 3)], 0, Direction::UnGraph);
        let mut v = vec![];
        for e in graph.neighbors(1) {
            v.push(e.target());
        }
        assert_eq!(v, vec![0, 2, 4, 3])
    }
    #[test]
    fn test_iter_unweighted() {
        let graph: ListGraph<usize> = ListGraph::unweighted_from(vec![(0, 1), (1, 2), (4, 1), (1, 3)], 0, Direction::UnGraph);
        let mut v = vec![];
        for &n in graph.neighbors_unweighted(1) {
            v.push(n);
        }
        assert_eq!(v, vec![0, 2, 4, 3])
    }

    #[test]
    fn test_diktstra_1() {
        let vec = vec![
            (0, 1, 1),
            (0, 2, 4),
            (1, 2, 2),
            (2, 3, 1),
            (1, 3, 5),
        ];
        let graph: ListGraph<isize> = ListGraph::weighted_from(vec, 0, Direction::DiGraph);
        // dbg!(&graph);
        let (w, prev_nodes) = diktstra(&graph, 0);
        assert_eq!(w, vec![0, 1, 3, 4]);
        let shortest_path03 = restore_path(0, 3, &prev_nodes);
        assert_eq!(shortest_path03, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_diktstra_2() {
        let vec = vec![
            (0, 1, 1),
            (0, 2, 4),
            (2, 0, 1),
            (1, 2, 2),
            (3, 1, 1),
            (3, 2, 5),
        ];
        let graph: ListGraph<isize> = ListGraph::weighted_from(vec, 0, Direction::DiGraph);
        // dbg!(&graph);
        let (w, prev_nodes) = diktstra(&graph, 1);
        assert_eq!(w, vec![3, 0, 2, std::isize::MAX]);
        assert_eq!(restore_path(1, 0, &prev_nodes), vec![1, 2, 0]);
    }

    #[test]
    fn test_dfs_1() {
        let vec = vec![
            (1, 2),
            (2, 4),
            (4, 3)
        ];

        let graph: UnweightedListGraph = ListGraph::unweighted_from(vec, 1, Direction::DiGraph);

        let res = dfs(0, &graph, DfsResultType::TimeStamp);
        assert_eq!(res.seen, vec![true, true, true, true]);
        assert_eq!(res.first_order, vec![0, 1, 3, 2]);
        assert_eq!(res.last_order, vec![7, 6, 4, 5]);
    }

    #[test]
    fn test_dfs_2() {
        let vec = vec![
            (1, 2),
            (1, 3),
            (2, 3),
            (2, 4),
            (3, 5),
            (4, 6),
            (5, 6),
        ];

        let graph: UnweightedListGraph = ListGraph::unweighted_from(vec, 1, Direction::DiGraph);
        let res = dfs(0, &graph, DfsResultType::TimeStamp);

        assert_eq!(res.seen, vec![true; graph.len()]);
        assert_eq!(res.first_order, vec![0, 1, 2, 8, 3, 4]);
        assert_eq!(res.last_order, vec![11, 10, 7, 9, 6, 5]);
    }
}
