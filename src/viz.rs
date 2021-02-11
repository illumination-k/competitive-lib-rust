// visualize graph


fn make_edge<S: ToString>(source: S, target: S, graph_type: &GraphType) -> String {
    match graph_type {
        GraphType::DiGraph => {
            format!("  {} -> {}", source.to_string(), target.to_string())
        },
        GraphType::UnGraph => {
            format!("  {} - {}", source.to_string(), target.to_string())
        },
    }
}
pub enum GraphType {
    DiGraph,
    UnGraph
}

pub fn dot(graph: Vec<Vec<usize>>, graph_type: GraphType) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    match graph_type {
        GraphType::DiGraph => {
            res.push("digraph example_graph {".to_string());
        },
        GraphType::UnGraph => {
            res.push("graph example_graph".to_string())
        }
    }

    for source in 0..graph.len() {
        for &target in graph[source].iter() {
            res.push(make_edge(source, target, &graph_type))
        }
    }

    res.push("}".to_string());

    res
}