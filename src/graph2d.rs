use cargo_snippet::snippet;
use std::collections::*;
use num_traits::*;
use std::ops::*;

#[derive(Debug, Clone)]
pub struct Graph2D<T> {
    graph: Vec<Vec<T>>,
    width: isize,
    height: isize
}

impl<K> Graph2D<K>
where K: Eq + Copy
{
    pub fn new(graph: Vec<Vec<K>>) -> Self {
        let width = graph[0].len() as isize;
        let height = graph.len() as isize;
        Self { graph, width, height }
    }

    pub fn is_in(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x >= self.width || y >= self.height
    }

    pub fn is_go(&self, x: isize, y: isize, obs: &Option<K>) -> Option<(usize, usize)> {
        if self.is_in(x, y) {
            return None
        }
        match obs {
            Some(obs) => { if &self[(x, y)] == obs { None } else { Some((x as usize, y as usize))}},
            None => Some((x as usize, y as usize)),
        }
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }
}

impl<T, K> Index<(T, T)> for Graph2D<K>
where T: PrimInt
{
    type Output = K;

    fn index<'a>(&'a self, index: (T, T)) -> &'a K {
        let x: usize = index.0.to_usize().expect("cannot convert usize. maybe negative number in x");
        let y: usize = index.1.to_usize().expect("cannot convert usize. maybe negative number in y");

        &self.graph[y][x]
    }
}

impl<T, K> IndexMut<(T, T)> for Graph2D<K>
where T: PrimInt
{
    fn index_mut<'a>(&'a mut self, index: (T, T)) -> &'a mut K {
        let x: usize = index.0.to_usize().expect("cannot convert usize. maybe negative number in x");
        let y: usize = index.1.to_usize().expect("cannot convert usize. maybe negative number in y");

        &mut self.graph[y][x]
    }
}

#[cfg(test)]
mod test {
    use super::Graph2D;

    #[test]
    fn test_index() {
        let mut graph = Graph2D::new(vec![vec![0; 10]; 10]);
        assert_eq!(graph[(1, 1)], 0);
        graph[(1, 1)] = 1;
        assert_eq!(graph[(1, 1)], 1);
    }
}

pub fn bfs2d<T: Eq + Copy>(graph: &Graph2D<T>, start: (usize, usize), obs: Option<T>) -> Graph2D<isize> {
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
                },
                None => {continue;}
            }
        }
    }

    dist
}


pub fn dfs2d<T: Eq + Copy>(graph: &Graph2D<T>, seen: &mut Vec<Vec<bool>>, start: (usize, usize), obs: Option<T>) {
    fn dfs<T: Eq + Copy>(
        graph: &Graph2D<T>, 
        start: (usize, usize), 
        obs: &Option<T>,
        directions: &Vec<(isize, isize)>, 
        seen: &mut Vec<Vec<bool>>,) {
            seen[start.1][start.0] = true;

            for &direction in directions.iter() {
                let next_x = start.0 as isize + direction.0;
                let next_y = start.1 as isize + direction.1;

                match graph.is_go(next_x, next_y, obs) {
                    Some(next) => {
                        if !seen[next.1][next.0] {
                            dfs(graph, next, obs, directions, seen)
                        }
                    },
                    None => { continue; }
                }
            }
    }
    
    // let mut seen = vec![vec![false; graph.width()]; graph.height()];
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    dfs(graph, start, &obs, &directions, seen);
}


#[cfg(test)]
mod test_bfs2d {
    // atcoder abc007-c
    use super::{Graph2D, bfs2d};

    fn solve(graph: Vec<Vec<char>>, s: (usize, usize), g: (usize, usize)) -> isize {
        let graph2d = Graph2D::new(graph);
        let dist = bfs2d(&graph2d, s, Some('#'));
        dist[g]
    } 

    #[test]
    fn test_bfs2d_1() {
        let graph: Vec<Vec<char>> = [
            "########",
            "#......#",
            "#.######",
            "#..#...#",
            "#..##..#",
            "##.....#",
            "########",
        ].iter().map(|x| x.chars().collect()).collect();

        let ans = solve(graph, (1, 1), (4, 3));
        assert_eq!(ans, 11);
    }

    #[test]
    fn test_bfs2d_2() {
        let graph: Vec<Vec<char>> = [
            "########",
            "#.#....#",
            "#.###..#",
            "#......#",
            "########"
        ].iter().map(|x| x.chars().collect()).collect();

        let ans = solve(graph, (1, 1), (3, 1));
        assert_eq!(ans, 10)
    }
}