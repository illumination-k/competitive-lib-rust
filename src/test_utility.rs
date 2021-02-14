use rand::distributions::{IndependentSample, Range};


pub fn make_random_unweighted_graph(node_number: usize, mut edge_number: usize, contain_loop: bool) -> Vec<(usize, usize)> {
    let between = Range::new(0, node_number);
    let mut rng = rand::thread_rng();

    let mut graph = vec![];
    while edge_number != 0 {
        let a = between.ind_sample(&mut rng);
        let b = between.ind_sample(&mut rng);
        if !contain_loop && a == b { continue; }
        graph.push((a, b));
        edge_number -= 1;
    }
    graph
}

pub fn make_random_isize_weighted_graph(node_number: usize, mut edge_number: usize, weight_range: (isize, isize), contain_loop: bool) -> Vec<(usize, usize, isize)> {
    let node_between = Range::new(0, node_number);
    let weight_between = Range::new(weight_range.0, weight_range.1);
    let mut rng = rand::thread_rng();
    let mut graph = vec![];
    while edge_number != 0 {
        let a = node_between.ind_sample(&mut rng);
        let b = node_between.ind_sample(&mut rng);
        let w = weight_between.ind_sample(&mut rng);
        if !contain_loop && a == b { continue; }
        graph.push((a, b, w));
        edge_number -= 1;
    }
    graph
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_make_graph() {
        let v = make_random_unweighted_graph(10, 10, false);
        assert_eq!(v.len(), 10);
        assert!(!v.iter().any(|(a, b)| a == b))
    } 
}