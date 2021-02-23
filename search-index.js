var searchIndex={};
searchIndex["competitive"] = {"doc":"competitive library for atcoder.","i":[[0,"binary_search","competitive","",null,null],[5,"binary_search","competitive::binary_search","",null,[[["t"]],["t"]]],[0,"dp","competitive","",null,null],[0,"dp","competitive::dp","",null,null],[0,"mod_int","competitive","",null,null],[3,"ModInt","competitive::mod_int","",null,null],[11,"value","","",0,[[["self"]],["t"]]],[11,"modulo","","",0,[[["self"]],["t"]]],[11,"new","","",0,[[["t"]],["self"]]],[11,"pow","","",0,[[["t"]],["self"]]],[0,"prime","competitive","",null,null],[3,"OsaK","competitive::prime","",null,null],[5,"is_prime","","",null,[[["numassign"],["primint"]],["bool"]]],[5,"enum_divisors","","",null,[[["numassign"],["primint"]],[["vec"],["numassign"],["primint"]]]],[5,"prime_factorize","","",null,[[["numassign"],["primint"]],["vec"]]],[5,"sieve_of_eratosthenes","","",null,[[["numcast"]],[["usize"],["vec",["usize"]]]]],[11,"new","","O(maxloglog(max))    construct osa-k from max size",1,[[["t"]],["self"]]],[11,"from","","O(max(vec)loglog(max(vec)))   construct osa-k from Vector",1,[[["vec"]],["self"]]],[11,"is_prime","","O(1) test x is prime or not",1,[[["self"],["t"]],["bool"]]],[11,"prime_factorize","","O(log(n))   prime factoraize ",1,[[["self"],["t"]],["hashmap"]]],[0,"graph2d","competitive","",null,null],[3,"Graph2D","competitive::graph2d","",null,null],[5,"bfs2d","","bfs of 2d graph",null,[[["eq"],["option"],["copy"],["graph2d"]],[["isize"],["graph2d",["isize"]]]]],[5,"dfs2d","","dfs of 2d graph",null,[[["copy"],["eq"],["vec"],["option"],["graph2d"]]]],[11,"new","","",2,[[["vec",["vec"]],["vec"]],["self"]]],[11,"is_not_in","","",2,[[["isize"],["self"]],["bool"]]],[11,"is_in","","",2,[[["isize"],["self"]],["bool"]]],[11,"is_go","","",2,[[["isize"],["option"],["self"]],["option"]]],[11,"width","","",2,[[["self"]],["usize"]]],[11,"height","","",2,[[["self"]],["usize"]]],[11,"row","","get Iterator of a row",2,[[["self"],["usize"]],["iter"]]],[11,"col","","get Iterator of a column",2,[[["self"],["usize"]]]],[11,"t","","",2,[[["self"]],["self"]]],[0,"graph","competitive","",null,null],[3,"Edge","competitive::graph","",null,null],[3,"ListGraph","","",null,null],[3,"DfsResults","","",null,null],[12,"seen","","",3,null],[12,"first_order","","",3,null],[12,"last_order","","",3,null],[12,"result_type","","",3,null],[4,"Direction","","",null,null],[13,"DiGraph","","",4,null],[13,"UnGraph","","",4,null],[4,"DfsResultType","","",null,null],[13,"FirstAndLastOrd","","",5,null],[13,"TimeStamp","","",5,null],[13,"NoOrd","","",5,null],[5,"diktstra","","Diktstra O(|E+V|log(|V|)) let E be edge number, let V be…",null,[[["listgraph"],["usize"]]]],[5,"bfs","","BFS O(|V+E|) let E be edge numbers, Let V be vertex…",null,[[["listgraph"],["usize"]]]],[5,"restore_path","","restore shortest path from start to goal",null,[[["vec"],["usize"]],[["usize"],["vec",["usize"]]]]],[5,"dfs","","DFS O(|V+E|) let E be edge numbers, Let V be vertex…",null,[[["dfsresulttype"],["listgraph"],["usize"]],["dfsresults"]]],[6,"UnweightedListGraph","","",null,null],[11,"new","","",6,[[["w"],["usize"]],["self"]]],[11,"new_unweighted","","",6,[[["usize"]],["self"]]],[11,"target","","",6,[[["self"]],["usize"]]],[11,"weight","","",6,[[["self"]],["w"]]],[11,"to_dot","","create dot format for graphviz `rust use…",7,[[["direction"],["self"],["bool"]],[["string"],["vec",["string"]]]]],[11,"new","","",7,[[["usize"]],["self"]]],[11,"unweighted_from","","create unweighted ListGraph. offset: index = val - offset…",7,[[["direction"],["usize"],["vec"]],["self"]]],[11,"weighted_from","","create weighted ListGraph. offset: index = val - offset…",7,[[["direction"],["usize"],["vec"]],["self"]]],[11,"t","","Reverse Direction of Graph",7,[[["self"]],["self"]]],[11,"len","","",7,[[["self"]],["usize"]]],[11,"neighbors","","",7,[[["self"],["usize"]],[["edge"],["iter",["edge"]]]]],[11,"neighbors_unweighted","","",7,[[["self"],["usize"]]]],[11,"new","","",3,[[["dfsresulttype"],["usize"]],["self"]]],[11,"update_first_order","","update first ord  ",3,[[["self"],["usize"]]]],[11,"update_last_order","","update last ord  ",3,[[["self"],["usize"]]]],[0,"combinations","competitive","",null,null],[3,"Combination","competitive::combinations","",null,null],[5,"combination","","simple calculation of combinations without modulo `use…",null,[[["usize"]],[["vec",["vec"]],["vec",["usize"]]]]],[5,"nck","","Simple wrapper of combinaiton `use…",null,[[["usize"]],["usize"]]],[5,"nhk","","Simple wrapper of combinations for multi choises",null,[[["usize"]],["usize"]]],[11,"new","","",8,[[["usize"]],["self"]]],[11,"fix_n","","",8,[[["self"],["usize"]]]],[11,"nck","","",8,[[["self"],["usize"]],["usize"]]],[11,"nhk","","",8,[[["self"],["usize"]],["usize"]]],[0,"math","competitive","",null,null],[5,"gcd","competitive::math","GCD",null,[[["primint"]],["primint"]]],[5,"lcm","","LCM",null,[[["primint"]],["primint"]]],[5,"gcd_list","","",null,[[["vec"],["primint"]],["primint"]]],[5,"lcm_list","","",null,[[["vec"],["primint"]],["primint"]]],[5,"quadratic_formula","","",null,[[["numcast"]],["option"]]],[5,"ext_gcd","","",null,[[["numcast"],["primint"]]]],[5,"inv_mod","","",null,[[["numcast"],["primint"]],[["numcast"],["primint"]]]],[5,"arithmetic_progression","","a0: the first term of serires d: common difference n:…",null,[[["primint"]],["primint"]]],[0,"scanner","competitive","",null,null],[3,"IO","competitive::scanner","",null,null],[11,"new","","",9,[[["w"],["r"]],["io"]]],[11,"write","","",9,[[["self"],["tostring"]]]],[11,"writeln","","",9,[[["self"],["tostring"]]]],[11,"write_vec","","",9,[[["self"],["tostring"],["vec"]]]],[11,"read","","",9,[[["self"]],["fromstr"]]],[11,"vec","","",9,[[["self"],["usize"]],[["vec"],["fromstr"]]]],[11,"vec2d","","",9,[[["self"],["usize"]],[["vec",["vec"]],["vec"]]]],[11,"set","","",9,[[["self"],["usize"]],[["fromstr"],["hashset"],["hash"],["eq"]]]],[11,"chars","","",9,[[["self"]],[["vec",["char"]],["char"]]]],[0,"data_structures","competitive","",null,null],[0,"segment_tree","competitive::data_structures","",null,null],[3,"SegmentTree","competitive::data_structures::segment_tree","Segment tree",null,null],[11,"new","","O(n). Construct segment tree for given size.",10,[[["usize"]],["self"]]],[11,"from_slice","","O(n). Construct segment tree from slice.",10,[[],["self"]]],[11,"len","","O(1). Length of sequence.",10,[[["self"]],["usize"]]],[11,"set","","O(log n). Set v to `i`-th element. `s[i] = v`",10,[[["self"],["usize"]]]],[11,"mappend","","O(log n). mappend v to `i`-th element `s[i] =…",10,[[["self"],["usize"]]]],[11,"get","","O(1). Get i-th element Equals to `query(i, i + 1)`",10,[[["self"],["usize"]],["t"]]],[11,"query","","O(log n). Query for `range`. Returns…",10,[[["self"]],["t"]]],[0,"union_find","competitive::data_structures","",null,null],[3,"UnionFind","competitive::data_structures::union_find","",null,null],[11,"new","","Create a new `UnionFind` of `n` disjoint sets.",11,[[["usize"]],["self"]]],[11,"find","","Return the representative for `x`.",11,[[["self"],["k"]],["k"]]],[11,"find_mut","","Return the representative for `x`.",11,[[["self"],["k"]],["k"]]],[11,"equiv","","Returns `true` if the given elements belong to the same…",11,[[["self"],["k"]],["bool"]]],[11,"union","","Unify the two sets containing `x` and `y`.",11,[[["self"],["k"]],["bool"]]],[11,"into_labeling","","Return a vector mapping each element to its representative.",11,[[],["vec"]]],[11,"size","","",11,[[["self"],["k"]],["usize"]]],[11,"member","","",11,[[["self"],["k"]],["hashset"]]],[11,"member_map","","",11,[[["self"]],[["hashset"],["hashmap",["hashset"]]]]],[0,"monoid","competitive::data_structures","",null,null],[3,"Sum","competitive::data_structures::monoid","",null,null],[12,"0","","",12,null],[3,"Product","","",null,null],[12,"0","","",13,null],[3,"Max","","",null,null],[12,"0","","",14,null],[3,"Min","","",null,null],[12,"0","","",15,null],[3,"XOR","","",null,null],[12,"0","","",16,null],[8,"Monoid","","",null,null],[10,"mempty","","単位元",17,[[],["self"]]],[10,"mappend","","op",17,[[["self"]],["self"]]],[0,"dfs_tree","competitive::data_structures","",null,null],[3,"DfsTree","competitive::data_structures::dfs_tree","graph: Tree pos: Termination timing of dfs",null,null],[11,"new","","",18,[[["vec",["vec"]],["vec",["usize"]]],["self"]]],[11,"build","","root: root of dfs_tree (tree_index) example `use…",18,[[["self"],["usize"]]]],[11,"subtree_range","","subtree range return [v, pos[v])",18,[[["self"],["usize"]]]],[11,"dfs_index","","",18,[[["self"],["usize"]],["usize"]]],[11,"tree_index","","",18,[[["self"],["usize"]],["usize"]]],[11,"pos","","",18,[[["self"],["usize"]],["usize"]]],[0,"fenwick_tree","competitive::data_structures","",null,null],[3,"FenwickTree","competitive::data_structures::fenwick_tree","In a max bit tree or Fenwick Tree, get(i) will return the…",null,null],[3,"MaxOp","","",null,null],[3,"SumOp","","",null,null],[6,"MaxBitTree","","Fenwick tree specialized for prefix-max",null,null],[6,"SumBitTree","","Fenwick tree specialized for prefix-sum",null,null],[8,"PrefixOp","","Fenwick tree prefix operator",null,null],[10,"operation","","",19,[[["t"]],["t"]]],[11,"new","","Create a new bit tree with len elements",20,[[["usize"]],["fenwicktree"]]],[11,"get","","Returns the largest element e that has been added to the…",20,[[["self"],["usize"]],["t"]]],[11,"set","","Set the value `val` at position `idx`; `val` will be…",20,[[["self"],["usize"],["t"]]]],[0,"geometry","competitive","",null,null],[0,"point","competitive::geometry","",null,null],[3,"Point","competitive::geometry::point","",null,null],[12,"x","","",21,null],[12,"y","","",21,null],[5,"triangle_space","","calculate space of triangle",null,[[["point"]],["t"]]],[5,"gradient","","calculate gradient   if zero division occur, return None",null,[[["point"]],["option"]]],[11,"new","","",21,[[["t"]],["self"]]],[11,"from_tuple","","",21,[[],["self"]]],[11,"rotaion","","roation point   0 <= theta <= 360",21,[[["self"],["t"]],["self"]]],[0,"format","competitive","",null,null],[8,"AtCoderFormat","competitive::format","Trait of format for atcoder",null,null],[10,"format","","",22,[[["self"]],["string"]]],[0,"test_utility","competitive","",null,null],[5,"make_random_unweighted_graph","competitive::test_utility","",null,[[["usize"],["bool"]],["vec"]]],[5,"make_random_weighted_graph","","",null,[[["bool"],["usize"]],["vec"]]],[5,"make_random_vec","","",null,[[["usize"]],["vec"]]],[11,"from","competitive::mod_int","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,[[["self"],["t"]]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"vzip","","",0,[[],["v"]]],[11,"from","competitive::prime","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,[[["self"],["t"]]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"vzip","","",1,[[],["v"]]],[11,"from","competitive::graph2d","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,[[["self"],["t"]]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"vzip","","",2,[[],["v"]]],[11,"from","competitive::graph","",6,[[["t"]],["t"]]],[11,"into","","",6,[[],["u"]]],[11,"to_owned","","",6,[[["self"]],["t"]]],[11,"clone_into","","",6,[[["self"],["t"]]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"try_into","","",6,[[],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"type_id","","",6,[[["self"]],["typeid"]]],[11,"vzip","","",6,[[],["v"]]],[11,"from","","",7,[[["t"]],["t"]]],[11,"into","","",7,[[],["u"]]],[11,"to_owned","","",7,[[["self"]],["t"]]],[11,"clone_into","","",7,[[["self"],["t"]]]],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"try_into","","",7,[[],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"type_id","","",7,[[["self"]],["typeid"]]],[11,"vzip","","",7,[[],["v"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[],["u"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,[[["self"],["t"]]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"try_into","","",3,[[],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"type_id","","",3,[[["self"]],["typeid"]]],[11,"vzip","","",3,[[],["v"]]],[11,"from","","",4,[[["t"]],["t"]]],[11,"into","","",4,[[],["u"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,[[["self"],["t"]]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"try_into","","",4,[[],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"type_id","","",4,[[["self"]],["typeid"]]],[11,"vzip","","",4,[[],["v"]]],[11,"from","","",5,[[["t"]],["t"]]],[11,"into","","",5,[[],["u"]]],[11,"to_owned","","",5,[[["self"]],["t"]]],[11,"clone_into","","",5,[[["self"],["t"]]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"try_into","","",5,[[],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"type_id","","",5,[[["self"]],["typeid"]]],[11,"vzip","","",5,[[],["v"]]],[11,"from","competitive::combinations","",8,[[["t"]],["t"]]],[11,"into","","",8,[[],["u"]]],[11,"to_owned","","",8,[[["self"]],["t"]]],[11,"clone_into","","",8,[[["self"],["t"]]]],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"try_into","","",8,[[],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"type_id","","",8,[[["self"]],["typeid"]]],[11,"vzip","","",8,[[],["v"]]],[11,"from","competitive::scanner","",9,[[["t"]],["t"]]],[11,"into","","",9,[[],["u"]]],[11,"try_from","","",9,[[["u"]],["result"]]],[11,"try_into","","",9,[[],["result"]]],[11,"borrow","","",9,[[["self"]],["t"]]],[11,"borrow_mut","","",9,[[["self"]],["t"]]],[11,"type_id","","",9,[[["self"]],["typeid"]]],[11,"vzip","","",9,[[],["v"]]],[11,"from","competitive::data_structures::segment_tree","",10,[[["t"]],["t"]]],[11,"into","","",10,[[],["u"]]],[11,"try_from","","",10,[[["u"]],["result"]]],[11,"try_into","","",10,[[],["result"]]],[11,"borrow","","",10,[[["self"]],["t"]]],[11,"borrow_mut","","",10,[[["self"]],["t"]]],[11,"type_id","","",10,[[["self"]],["typeid"]]],[11,"vzip","","",10,[[],["v"]]],[11,"from","competitive::data_structures::union_find","",11,[[["t"]],["t"]]],[11,"into","","",11,[[],["u"]]],[11,"to_owned","","",11,[[["self"]],["t"]]],[11,"clone_into","","",11,[[["self"],["t"]]]],[11,"try_from","","",11,[[["u"]],["result"]]],[11,"try_into","","",11,[[],["result"]]],[11,"borrow","","",11,[[["self"]],["t"]]],[11,"borrow_mut","","",11,[[["self"]],["t"]]],[11,"type_id","","",11,[[["self"]],["typeid"]]],[11,"vzip","","",11,[[],["v"]]],[11,"from","competitive::data_structures::monoid","",12,[[["t"]],["t"]]],[11,"from","","",12,[[],["t"]]],[11,"into","","",12,[[],["u"]]],[11,"to_owned","","",12,[[["self"]],["t"]]],[11,"clone_into","","",12,[[["self"],["t"]]]],[11,"try_from","","",12,[[["u"]],["result"]]],[11,"try_into","","",12,[[],["result"]]],[11,"borrow","","",12,[[["self"]],["t"]]],[11,"borrow_mut","","",12,[[["self"]],["t"]]],[11,"type_id","","",12,[[["self"]],["typeid"]]],[11,"vzip","","",12,[[],["v"]]],[11,"from","","",13,[[["t"]],["t"]]],[11,"from","","",13,[[],["t"]]],[11,"into","","",13,[[],["u"]]],[11,"to_owned","","",13,[[["self"]],["t"]]],[11,"clone_into","","",13,[[["self"],["t"]]]],[11,"try_from","","",13,[[["u"]],["result"]]],[11,"try_into","","",13,[[],["result"]]],[11,"borrow","","",13,[[["self"]],["t"]]],[11,"borrow_mut","","",13,[[["self"]],["t"]]],[11,"type_id","","",13,[[["self"]],["typeid"]]],[11,"vzip","","",13,[[],["v"]]],[11,"from","","",14,[[["t"]],["t"]]],[11,"from","","",14,[[],["t"]]],[11,"into","","",14,[[],["u"]]],[11,"to_owned","","",14,[[["self"]],["t"]]],[11,"clone_into","","",14,[[["self"],["t"]]]],[11,"try_from","","",14,[[["u"]],["result"]]],[11,"try_into","","",14,[[],["result"]]],[11,"borrow","","",14,[[["self"]],["t"]]],[11,"borrow_mut","","",14,[[["self"]],["t"]]],[11,"type_id","","",14,[[["self"]],["typeid"]]],[11,"vzip","","",14,[[],["v"]]],[11,"from","","",15,[[["t"]],["t"]]],[11,"from","","",15,[[],["t"]]],[11,"into","","",15,[[],["u"]]],[11,"to_owned","","",15,[[["self"]],["t"]]],[11,"clone_into","","",15,[[["self"],["t"]]]],[11,"try_from","","",15,[[["u"]],["result"]]],[11,"try_into","","",15,[[],["result"]]],[11,"borrow","","",15,[[["self"]],["t"]]],[11,"borrow_mut","","",15,[[["self"]],["t"]]],[11,"type_id","","",15,[[["self"]],["typeid"]]],[11,"vzip","","",15,[[],["v"]]],[11,"from","","",16,[[["t"]],["t"]]],[11,"from","","",16,[[],["t"]]],[11,"into","","",16,[[],["u"]]],[11,"to_owned","","",16,[[["self"]],["t"]]],[11,"clone_into","","",16,[[["self"],["t"]]]],[11,"try_from","","",16,[[["u"]],["result"]]],[11,"try_into","","",16,[[],["result"]]],[11,"borrow","","",16,[[["self"]],["t"]]],[11,"borrow_mut","","",16,[[["self"]],["t"]]],[11,"type_id","","",16,[[["self"]],["typeid"]]],[11,"vzip","","",16,[[],["v"]]],[11,"from","competitive::data_structures::dfs_tree","",18,[[["t"]],["t"]]],[11,"into","","",18,[[],["u"]]],[11,"to_owned","","",18,[[["self"]],["t"]]],[11,"clone_into","","",18,[[["self"],["t"]]]],[11,"try_from","","",18,[[["u"]],["result"]]],[11,"try_into","","",18,[[],["result"]]],[11,"borrow","","",18,[[["self"]],["t"]]],[11,"borrow_mut","","",18,[[["self"]],["t"]]],[11,"type_id","","",18,[[["self"]],["typeid"]]],[11,"vzip","","",18,[[],["v"]]],[11,"from","competitive::data_structures::fenwick_tree","",20,[[["t"]],["t"]]],[11,"into","","",20,[[],["u"]]],[11,"try_from","","",20,[[["u"]],["result"]]],[11,"try_into","","",20,[[],["result"]]],[11,"borrow","","",20,[[["self"]],["t"]]],[11,"borrow_mut","","",20,[[["self"]],["t"]]],[11,"type_id","","",20,[[["self"]],["typeid"]]],[11,"vzip","","",20,[[],["v"]]],[11,"from","","",23,[[["t"]],["t"]]],[11,"into","","",23,[[],["u"]]],[11,"try_from","","",23,[[["u"]],["result"]]],[11,"try_into","","",23,[[],["result"]]],[11,"borrow","","",23,[[["self"]],["t"]]],[11,"borrow_mut","","",23,[[["self"]],["t"]]],[11,"type_id","","",23,[[["self"]],["typeid"]]],[11,"vzip","","",23,[[],["v"]]],[11,"from","","",24,[[["t"]],["t"]]],[11,"into","","",24,[[],["u"]]],[11,"try_from","","",24,[[["u"]],["result"]]],[11,"try_into","","",24,[[],["result"]]],[11,"borrow","","",24,[[["self"]],["t"]]],[11,"borrow_mut","","",24,[[["self"]],["t"]]],[11,"type_id","","",24,[[["self"]],["typeid"]]],[11,"vzip","","",24,[[],["v"]]],[11,"from","competitive::geometry::point","",21,[[["t"]],["t"]]],[11,"into","","",21,[[],["u"]]],[11,"to_owned","","",21,[[["self"]],["t"]]],[11,"clone_into","","",21,[[["self"],["t"]]]],[11,"try_from","","",21,[[["u"]],["result"]]],[11,"try_into","","",21,[[],["result"]]],[11,"borrow","","",21,[[["self"]],["t"]]],[11,"borrow_mut","","",21,[[["self"]],["t"]]],[11,"type_id","","",21,[[["self"]],["typeid"]]],[11,"vzip","","",21,[[],["v"]]],[11,"mempty","competitive::data_structures::monoid","",12,[[],["self"]]],[11,"mappend","","",12,[[["self"]],["self"]]],[11,"mempty","","",13,[[],["self"]]],[11,"mappend","","",13,[[["self"]],["self"]]],[11,"mempty","","",14,[[],["self"]]],[11,"mappend","","",14,[[["self"]],["self"]]],[11,"mempty","","",15,[[],["self"]]],[11,"mappend","","",15,[[["self"]],["self"]]],[11,"mempty","","",16,[[],["self"]]],[11,"mappend","","",16,[[["self"]],["self"]]],[11,"operation","competitive::data_structures::fenwick_tree","",23,[[["t"]],["t"]]],[11,"operation","","",24,[[["t"]],["t"]]],[11,"from","competitive::data_structures::monoid","",12,[[["t"]],["self"]]],[11,"from","","",13,[[["t"]],["self"]]],[11,"from","","",14,[[["t"]],["self"]]],[11,"from","","",15,[[["t"]],["self"]]],[11,"from","","",16,[[["t"]],["self"]]],[11,"clone","competitive::mod_int","",0,[[["self"]],["self"]]],[11,"clone","competitive::prime","",1,[[["self"]],["osak"]]],[11,"clone","competitive::graph2d","",2,[[["self"]],["graph2d"]]],[11,"clone","competitive::graph","",6,[[["self"]],["edge"]]],[11,"clone","","",7,[[["self"]],["listgraph"]]],[11,"clone","","",4,[[["self"]],["direction"]]],[11,"clone","","",5,[[["self"]],["dfsresulttype"]]],[11,"clone","","",3,[[["self"]],["dfsresults"]]],[11,"clone","competitive::combinations","",8,[[["self"]],["combination"]]],[11,"clone","competitive::data_structures::union_find","",11,[[["self"]],["unionfind"]]],[11,"clone","competitive::data_structures::monoid","",12,[[["self"]],["sum"]]],[11,"clone","","",13,[[["self"]],["product"]]],[11,"clone","","",14,[[["self"]],["max"]]],[11,"clone","","",15,[[["self"]],["min"]]],[11,"clone","","",16,[[["self"]],["xor"]]],[11,"clone","competitive::data_structures::dfs_tree","",18,[[["self"]],["dfstree"]]],[11,"clone","competitive::geometry::point","",21,[[["self"]],["point"]]],[11,"default","competitive::data_structures::monoid","",12,[[],["sum"]]],[11,"default","","",13,[[],["product"]]],[11,"default","","",14,[[],["max"]]],[11,"default","","",15,[[],["min"]]],[11,"default","","",16,[[],["xor"]]],[11,"cmp","competitive::graph2d","",2,[[["self"],["graph2d"]],["ordering"]]],[11,"cmp","competitive::graph","",6,[[["self"]],["ordering"]]],[11,"eq","competitive::graph2d","",2,[[["self"],["graph2d"]],["bool"]]],[11,"ne","","",2,[[["self"],["graph2d"]],["bool"]]],[11,"eq","competitive::graph","",6,[[["self"],["edge"]],["bool"]]],[11,"ne","","",6,[[["self"],["edge"]],["bool"]]],[11,"eq","","",5,[[["dfsresulttype"],["self"]],["bool"]]],[11,"eq","competitive::data_structures::monoid","",14,[[["self"],["max"]],["bool"]]],[11,"ne","","",14,[[["self"],["max"]],["bool"]]],[11,"eq","competitive::geometry::point","",21,[[["point"],["self"]],["bool"]]],[11,"ne","","",21,[[["point"],["self"]],["bool"]]],[11,"partial_cmp","competitive::graph2d","",2,[[["self"],["graph2d"]],[["ordering"],["option",["ordering"]]]]],[11,"lt","","",2,[[["self"],["graph2d"]],["bool"]]],[11,"le","","",2,[[["self"],["graph2d"]],["bool"]]],[11,"gt","","",2,[[["self"],["graph2d"]],["bool"]]],[11,"ge","","",2,[[["self"],["graph2d"]],["bool"]]],[11,"partial_cmp","competitive::graph","",6,[[["self"]],[["option",["ordering"]],["ordering"]]]],[11,"partial_cmp","competitive::geometry::point","",21,[[["point"],["self"]],[["ordering"],["option",["ordering"]]]]],[11,"lt","","",21,[[["point"],["self"]],["bool"]]],[11,"le","","",21,[[["point"],["self"]],["bool"]]],[11,"gt","","",21,[[["point"],["self"]],["bool"]]],[11,"ge","","",21,[[["point"],["self"]],["bool"]]],[11,"fmt","competitive::mod_int","",0,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::prime","",1,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::graph2d","",2,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::graph","",6,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",7,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",4,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",5,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",3,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::combinations","",8,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::data_structures::segment_tree","",10,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::data_structures::union_find","",11,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::data_structures::monoid","",12,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",13,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",14,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",15,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",16,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::data_structures::dfs_tree","",18,[[["formatter"],["self"]],["result"]]],[11,"fmt","competitive::geometry::point","",21,[[["formatter"],["self"]],["result"]]],[11,"div","competitive::mod_int","",0,[[["modint"]]]],[11,"div","","",0,[[["t"]]]],[11,"div","competitive::geometry::point","",21,[[["t"]],["self"]]],[11,"div","","",21,[[],["self"]]],[11,"sub","competitive::mod_int","",0,[[["t"]]]],[11,"sub","","",0,[[["modint"]]]],[11,"sub","competitive::geometry::point","",21,[[["t"]],["self"]]],[11,"sub","","",21,[[],["self"]]],[11,"add","competitive::mod_int","",0,[[["t"]]]],[11,"add","","",0,[[["modint"]]]],[11,"add","competitive::geometry::point","",21,[[["t"]],["self"]]],[11,"add","","",21,[[],["self"]]],[11,"mul","competitive::mod_int","",0,[[["t"]]]],[11,"mul","","",0,[[["modint"]]]],[11,"mul","competitive::geometry::point","",21,[[["t"]],["self"]]],[11,"mul","","",21,[[],["self"]]],[11,"add_assign","competitive::mod_int","",0,[[["self"],["t"]]]],[11,"add_assign","","",0,[[["modint"],["self"]]]],[11,"add_assign","competitive::geometry::point","",21,[[["self"],["t"]]]],[11,"add_assign","","",21,[[["self"]]]],[11,"sub_assign","competitive::mod_int","",0,[[["self"],["t"]]]],[11,"sub_assign","","",0,[[["modint"],["self"]]]],[11,"sub_assign","competitive::geometry::point","",21,[[["self"],["t"]]]],[11,"sub_assign","","",21,[[["self"]]]],[11,"mul_assign","competitive::mod_int","",0,[[["self"],["t"]]]],[11,"mul_assign","","",0,[[["modint"],["self"]]]],[11,"mul_assign","competitive::geometry::point","",21,[[["self"],["t"]]]],[11,"mul_assign","","",21,[[["self"]]]],[11,"div_assign","competitive::mod_int","",0,[[["self"],["t"]]]],[11,"div_assign","","",0,[[["modint"],["self"]]]],[11,"div_assign","competitive::geometry::point","",21,[[["self"],["t"]]]],[11,"div_assign","","",21,[[["self"]]]],[11,"index","competitive::graph2d","",2,[[["self"]],["k"]]],[11,"index","competitive::graph","",7,[[["self"],["usize"]]]],[11,"index_mut","competitive::graph2d","",2,[[["self"]],["k"]]],[11,"index_mut","competitive::graph","",7,[[["self"],["usize"]],["vec"]]]],"p":[[3,"ModInt"],[3,"OsaK"],[3,"Graph2D"],[3,"DfsResults"],[4,"Direction"],[4,"DfsResultType"],[3,"Edge"],[3,"ListGraph"],[3,"Combination"],[3,"IO"],[3,"SegmentTree"],[3,"UnionFind"],[3,"Sum"],[3,"Product"],[3,"Max"],[3,"Min"],[3,"XOR"],[8,"Monoid"],[3,"DfsTree"],[8,"PrefixOp"],[3,"FenwickTree"],[3,"Point"],[8,"AtCoderFormat"],[3,"MaxOp"],[3,"SumOp"]]};
addSearchOptions(searchIndex);initSearch(searchIndex);