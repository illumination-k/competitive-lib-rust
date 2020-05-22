use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Graph2D<T, I> {
    graph2d: Vec<Vec<T>>,
    dequeue: VecDeque<(i64, i64)>,
    dist: Vec<Vec<I>>,
    x: Vec<i64>,
    y: Vec<i64>,
}