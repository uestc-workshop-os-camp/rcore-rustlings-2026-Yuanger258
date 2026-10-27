/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,  //邻接表，是一个hash表 k是一个String类型，v是vec里面放的元组
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {          //新建一个无向图，就是用邻接表储存无向图
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {   //给邻接表中加入一条边，边的话是包含两个结点和一个权重
        //TODO
        let (node1,node2,weight) = edge;
       
        self.add_node(node1);
        self.add_node(node2);

        match self.adjacency_table_mutable().get_mut(node1) {
            Some(v1) =>{
                v1.push((String::from(node2),weight));
            },
            None => (),
        }
         match self.adjacency_table_mutable().get_mut(node2) {
            Some(v2) =>{
                v2.push((String::from(node1),weight));
            },
            None => (),
        }


    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {    //加入结点，就是结点的名字
        //TODO
        //加入结点，首先判断一下结点是否存在，如果已经存在则false
        //就是在邻接表中先插入一个键值对，k就是String类型，值的话先插入一个空的vec，添加边的时候再继续处理vec
        match self.contains(node) {
            true => false,
            false =>{
                self.adjacency_table_mutable().insert(String::from(node),Vec::new());
                true
            },
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) { //添加边
        //TODO
        let (node1,node2,weight) = edge;
        //然后添加两个结点，不需要管结点原来是否存在
        self.add_node(node1);
        self.add_node(node2);
        //然后实现添加边
        match self.adjacency_table_mutable().get_mut(node1) {
            Some(v1) =>{
                v1.push((String::from(node2),weight));
            },
            None => (),
        }
         match self.adjacency_table_mutable().get_mut(node2) {
            Some(v2) =>{
                v2.push((String::from(node1),weight));
            },
            None => (),
        }
    }
    fn contains(&self, node: &str) -> bool {    //这里是检查结点是否已经包含在图里
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {             //收集所有节点
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {       //返回vec，里面是元组，包含两个节点和一个权
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {                                 //这里测试了add_edge 
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}