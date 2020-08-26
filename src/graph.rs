use cargo_snippet::snippet;
use std::collections::*;
use num_traits::*;


#[derive(Debug, Clone)]
pub struct Graph2D<T> {
    graph: Vec<Vec<T>>,
    width: isize,
    height: isize
}

impl<T: Eq> Graph2D<T> {
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
}

pub fn bfs2d<T: Eq>(graph: Graph2D<T>, start: (usize, usize), obs: Option<T>) -> Vec<Vec<isize>> {
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

