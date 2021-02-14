initSidebarItems({"enum":[["DfsResultType",""],["Direction",""]],"fn":[["bfs","BFS   O(|V+E|)   let E be edge numbers, Let V be vertex numbers return distance from start and prev nodes information restore_path function can create shortest path from start to goal from prev nodes. `rust use competitive::graph::*; let vec = vec![     (1, 2),     (1, 4),     (2, 4),     (4, 3)     ]; let graph: UnweightedListGraph = ListGraph::unweighted_from(vec, 4, 1, Direction::DiGraph); let (dist, prev_nodes) = bfs(&graph, 0); assert_eq!(dist, vec![0, 1, 2, 1]); assert_eq!(restore_path(0, 2, &prev_nodes), vec![0, 3, 2]); `"],["dfs","DFS   O(|V+E|)   let E be edge numbers, Let V be vertex numbers. return the DfsResults struct, which has seen results and first and last ord or time stamps."],["diktstra","Diktstra    O(|E+V|log(|V|))    let E be edge number, let V be vertex number. return distance from start and prev nodes information. restore_path function can create shortest path from start to goal from prev nodes. ```rust use competitive::graph::*; let vec = vec![     (0, 1, 1),     (0, 2, 4),     (2, 0, 1),     (1, 2, 2),     (3, 1, 1),     (3, 2, 5), ]; let graph: ListGraph = ListGraph::weighted_from(vec, 4, 0, Direction::DiGraph);"],["restore_path","restore shortest path from start to goal"]],"struct":[["DfsResults",""],["Edge",""],["ListGraph",""]],"type":[["UnweightedListGraph",""]]});