use cargo_snippet::snippet;
use std::collections::*;

#[derive(Debug, Clone)]
pub struct Graph2D<T> {
    graph: Vec<Vec<T>>,
    width: isize,
    height: isize
}

impl<T: Eq + Copy> Graph2D<T> {
    pub fn new(graph: Vec<Vec<T>>) -> Self {
        let width = graph[0].len() as isize;
        let height = graph.len() as isize;
        Self { graph, width, height }
    }

    pub fn is_go(&self, x: isize, y: isize, obs: &Option<T>) -> Option<(usize, usize)> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None
        }
        match obs {
            Some(obs) => { if self.graph[y as usize][x as usize] == *obs { None } else { Some((x as usize, y as usize))}},
            None => Some((x as usize, y as usize)),
        }
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    pub fn index(&self, index: (usize, usize)) -> T {
        self.graph[index.1][index.0]
    }
}

pub fn bfs2d<T: Eq + Copy>(graph: &Graph2D<T>, start: (usize, usize), obs: Option<T>) -> Vec<Vec<isize>> {
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dist: Vec<Vec<isize>> = vec![vec![-1; graph.width()]; graph.height()];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    let (sx, sy): (usize, usize) = start;
    dist[sy][sx] = 0;
    queue.push_back(start);

    while !queue.is_empty() {
        let (cx, cy): (usize, usize) = queue.pop_front().unwrap();

        for direction in directions.iter() {
            let next_x = cx as isize + direction.0;
            let next_y = cy as isize + direction.1;

            match graph.is_go(next_x, next_y, &obs) {
                Some(next) => {
                    if dist[next.1][next.0] == -1 {
                        dist[next.1][next.0] = dist[cy][cx] + 1;
                        queue.push_back(next)
                    }
                },
                None => {continue;}
            }
        }
    }

    dist
}

pub fn dfs2d<T: Eq + Copy>(graph: Graph2D<T>, start: (usize, usize), obs: Option<T>) -> Vec<Vec<bool>> {
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
                            dfs(graph, start, obs, directions, seen)
                        }
                    },
                    None => { continue; }
                }
            }
    }
    let mut seen = vec![vec![false; graph.width()]; graph.height()];
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    dfs(&graph, start, &obs, &directions, &mut seen);

    seen
}