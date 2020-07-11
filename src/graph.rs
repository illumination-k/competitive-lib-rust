use std::collections::VecDeque;
use num_traits::*;

#[derive(Debug, Clone)]
pub struct MatGraph<T, I> {
    matgraph: Vec<Vec<T>>,
    dequeue: VecDeque<(i64, i64)>,
    dist: Vec<Vec<I>>,
    x: Vec<i64>,
    y: Vec<i64>,
}
