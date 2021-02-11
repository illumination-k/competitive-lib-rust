pub mod listgraph {
    use num_traits::{NumCast, One, Zero};
    use std::{collections::BinaryHeap, fmt, ops::{Index, IndexMut}, writeln};
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
        fn neighbors(&self, source: usize) -> NeighborhoodIter<W>;
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

    pub fn diktstra<W>(graph: &ListGraph<W>, start: usize) -> (Vec<W>, Vec<usize>) 
        where W: Copy + One + Zero + PartialEq + PartialOrd + NumCast
    {
        let mut dist = vec![W::from(1<<50).unwrap(); graph.len()];
        dist[start] = W::zero();

        let mut bq = BinaryHeap::new();

        let mut prev_nodes = vec![std::usize::MAX; graph.len()];

        // add dumy edge and start position
        bq.push((Edge::new(0, W::one()), start));

        while let Some((edge, position)) = bq.pop() {
            for &e in graph.neighbors(position) {
                if edge.weight() > dist[position] { continue; }
                bq.push((e, e.target()));
                dist[edge.target()] = dist[position] + edge.weight();
                prev_nodes[edge.target()] = position;
            }
        }

        (dist, prev_nodes)
    }

    pub fn restore_path(start: usize, goal: usize, prev_nodes: &Vec<usize>) -> Vec<usize> {
        let mut res = vec![goal];
        let mut pos = start;
        while pos != start {
            pos = prev_nodes[pos];
            res.push(pos)
        }
        res
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
        for n in graph.neighbors(1) {
            v.push(n.target());
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
}
