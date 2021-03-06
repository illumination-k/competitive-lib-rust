#[allow(unused_imports)]
use std::collections::*;
use std::fmt;
use std::{ops::*, writeln};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Graph2D<T> {
    graph: Vec<Vec<T>>,
    width: isize,
    height: isize,
}

impl<T> fmt::Debug for Graph2D<T>
where
    T: ToString + Eq + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "width = {}, height = {}", self.width, self.height)?;
        for y in 0..self.height() {
            let s: String = self.graph[y]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(f, "{}", s)?;
        }

        Ok(())
    }
}

impl<K> Graph2D<K>
where
    K: Eq + Copy,
{
    pub fn new(graph: Vec<Vec<K>>) -> Self {
        let width = graph[0].len() as isize;
        let height = graph.len() as isize;
        Self {
            graph,
            width,
            height,
        }
    }

    pub fn is_not_in(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x >= self.width || y >= self.height
    }

    pub fn is_in(&self, x: isize, y: isize) -> bool {
        !(self.is_not_in(x, y))
    }

    pub fn is_go(&self, x: isize, y: isize, obs: &Option<K>) -> Option<(usize, usize)> {
        if self.is_not_in(x, y) {
            return None;
        }
        match obs {
            Some(obs) => {
                if &self[(x as usize, y as usize)] == obs {
                    None
                } else {
                    Some((x as usize, y as usize))
                }
            }
            None => Some((x as usize, y as usize)),
        }
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    /// get Iterator of a row
    pub fn row(&self, y: usize) -> std::slice::Iter<K> {
        self.graph[y].iter()
    }

    /// get Iterator of a column
    pub fn col(&self, x: usize) -> impl Iterator<Item = K> + '_ {
        let height = self.height();
        (0..height).map(move |y| self[(x, y)])
    }

    // transpose
    pub fn t(&self) -> Self {
        let mut vec = vec![vec![]; self.width()];

        for i in 0..self.width() {
            for x in self.col(i) {
                vec[i].push(x);
            }
        }
        Graph2D::new(vec)
    }
}

impl<K> Index<(usize, usize)> for Graph2D<K> {
    type Output = K;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a K {
        let (x, y): (usize, usize) = index;

        &self.graph[y][x]
    }
}

impl<K> IndexMut<(usize, usize)> for Graph2D<K> {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut K {
        let (x, y): (usize, usize) = index;

        &mut self.graph[y][x]
    }
}

/// bfs of 2d graph
pub fn bfs2d<T: Eq + Copy>(
    graph: &Graph2D<T>,
    start: (usize, usize),
    obs: Option<T>,
) -> Graph2D<isize> {
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dist: Graph2D<isize> = Graph2D::new(vec![vec![-1; graph.width()]; graph.height()]);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    dist[start] = 0;
    queue.push_back(start);

    while let Some((cx, cy)) = queue.pop_front() {
        for direction in directions.iter() {
            let next_x = cx as isize + direction.0;
            let next_y = cy as isize + direction.1;

            match graph.is_go(next_x, next_y, &obs) {
                Some(next) => {
                    if dist[next] == -1 {
                        dist[next] = dist[(cx, cy)] + 1;
                        queue.push_back(next)
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }

    dist
}

/// dfs of 2d graph
pub fn dfs2d<T: Eq + Copy>(
    graph: &Graph2D<T>,
    seen: &mut Vec<Vec<bool>>,
    start: (usize, usize),
    obs: Option<T>,
) {
    fn dfs<T: Eq + Copy>(
        graph: &Graph2D<T>,
        start: (usize, usize),
        obs: &Option<T>,
        directions: &Vec<(isize, isize)>,
        seen: &mut Vec<Vec<bool>>,
    ) {
        seen[start.1][start.0] = true;

        for &direction in directions.iter() {
            let next_x = start.0 as isize + direction.0;
            let next_y = start.1 as isize + direction.1;

            match graph.is_go(next_x, next_y, obs) {
                Some(next) => {
                    if !seen[next.1][next.0] {
                        dfs(graph, next, obs, directions, seen)
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }

    // let mut seen = vec![vec![false; graph.width()]; graph.height()];
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    dfs(graph, start, &obs, &directions, seen);
}

#[cfg(test)]
mod test_graph2d {
    use super::Graph2D;

    fn make_graph2d() -> Graph2D<usize> {
        let graph: Vec<Vec<usize>> = vec![(0..10).collect(); 10];
        Graph2D::new(graph)
    }

    #[test]
    fn test_index() {
        let mut graph = Graph2D::new(vec![vec![0; 10]; 10]);
        assert_eq!(graph[(1, 1)], 0);
        graph[(1, 1)] = 1;
        assert_eq!(graph[(1, 1)], 1);
    }

    #[test]
    fn test_col() {
        let graph = make_graph2d();
        for i in graph.col(1) {
            assert_eq!(i, 1)
        }
    }
}

#[cfg(test)]
mod test_bfs2d {
    // atcoder abc007-c
    use super::{bfs2d, Graph2D};

    fn solve(graph: Vec<Vec<char>>, s: (usize, usize), g: (usize, usize)) -> isize {
        let graph2d = Graph2D::new(graph);
        let dist = bfs2d(&graph2d, s, Some('#'));
        dist[g]
    }

    #[test]
    fn test_bfs2d_1() {
        let graph: Vec<Vec<char>> = [
            "########", "#......#", "#.######", "#..#...#", "#..##..#", "##.....#", "########",
        ]
        .iter()
        .map(|x| x.chars().collect())
        .collect();

        let ans = solve(graph, (1, 1), (4, 3));
        assert_eq!(ans, 11);
    }

    #[test]
    fn test_bfs2d_2() {
        let graph: Vec<Vec<char>> = ["########", "#.#....#", "#.###..#", "#......#", "########"]
            .iter()
            .map(|x| x.chars().collect())
            .collect();

        let ans = solve(graph, (1, 1), (3, 1));
        assert_eq!(ans, 10)
    }
}

#[cfg(test)]
mod test_dfs2d {
    use super::{dfs2d, Graph2D};

    fn solve(graph: Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize)) -> bool {
        let graph2d = Graph2D::new(graph);
        let mut seen = vec![vec![false; graph2d.width()]; graph2d.height()];

        dfs2d(&graph2d, &mut seen, start, Some('#'));
        seen[goal.1][goal.0]
    }

    #[test]
    fn test_dfs2d_1() {
        let graph: Vec<Vec<char>> = ["s####", "....#", "#####", "#...g"]
            .iter()
            .map(|x| x.chars().collect())
            .collect();

        let start = (0, 0);
        let goal = (4, 3);

        let ans = solve(graph, start, goal);
        assert_eq!(ans, false)
    }

    #[test]
    fn test_dfs2d_2() {
        let graph: Vec<Vec<char>> = [
            "s.........",
            "#########.",
            "#.......#.",
            "#..####.#.",
            "##....#.#.",
            "#####.#.#.",
            "g.#.#.#.#.",
            "#.#.#.#.#.",
            "###.#.#.#.",
            "#.....#...",
        ]
        .iter()
        .map(|x| x.chars().collect())
        .collect();

        let start = (0, 0);
        let goal = (0, 6);

        let ans = solve(graph, start, goal);
        assert_eq!(ans, false);
    }

    #[test]
    fn test_dfs2d_3() {
        let graph: Vec<Vec<char>> = [
            "s.........",
            "#########.",
            "#.......#.",
            "#..####.#.",
            "##....#.#.",
            "#####.#.#.",
            "g.#.#.#.#.",
            "#.#.#.#.#.",
            "#.#.#.#.#.",
            "#.....#...",
        ]
        .iter()
        .map(|x| x.chars().collect())
        .collect();

        let start = (0, 0);
        let goal = (0, 6);

        let ans = solve(graph, start, goal);
        assert_eq!(ans, true);
    }
}
