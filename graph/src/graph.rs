use std::collections::{HashSet, HashMap};

/// 有向图作为基类
///  对于无向图, 无向边要为两个方向加边
trait Graph {
    /// 使用vec模拟邻接表的"链"
    ///  TIPS: 哪些是抽象是可以复用, 哪些是不行的?? 无向图就是一次添加两个方向的有向图,
    ///  所以可以用有向图作为基类
    fn adjacency_table_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;


    /// 添加边: (from, to, weight)
    fn add_edge(&mut self, pair: (&str, &str, i32)) {
        // 节点添加到点集中
        self.add_node(pair.0);
        self.add_node(pair.1);
        self.adjacency_table_mut()
            .entry(pair.0.to_string())
            .and_modify(|list| list.push((pair.1.to_string(), pair.2))) // 插入邻接表
            .or_insert(vec![(pair.1.to_string(), pair.2)]);         // 如果邻接表不存在则先创建
    }

    /// 向图的节点集合中添加节点
    fn add_node(&mut self, node: &str) {
        self.adjacency_table_mut()
            .entry(node.to_string())
            .or_insert(vec![]);
    }

    /// 返回所有邻接点的key和权重: (key, weight)
    /// 直接返回邻接表vec的引用
    fn neighbours(&self, node: &str) -> Option<&Vec<(String, i32)>> {
        match self.adjacency_table().get(&node.to_string()) {
            Some(list) => Some(list),
            None => None,
        }
    }

    /// 返回所有边
    ///  TIPS: 为什么要用引用?? -> 因为不能夺取所有权
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut ret = vec![];
        for (from, to) in self.adjacency_table().iter() {
            for t in to {
                ret.push((from, &t.0, t.1));
            }
        }
        ret
    }

    /// 返回图中所有节点的集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    /// 判断图是否包含某节点
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
}

struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,  // key: Vec<(node, weight)>
}

impl UndirectedGraph {
    pub fn new() -> Self {
        Self { adjacency_table: HashMap::new() }
    }
}

impl Graph for UndirectedGraph {
    fn adjacency_table_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, pair: (&str, &str, i32)) { 
        // 节点添加到点集中
        self.add_node(pair.0);
        self.add_node(pair.1);
        let table = self.adjacency_table_mut();
        table.entry(pair.0.to_string())
            .and_modify(|list| list.push((pair.1.to_string(), pair.2))) // 插入邻接表
            .or_insert(vec![(pair.1.to_string(), pair.2)]);         // 如果邻接表不存在则先创建
        table.entry(pair.1.to_string())
            .and_modify(|list| list.push((pair.0.to_string(), pair.2))) // 插入邻接表
            .or_insert(vec![(pair.0.to_string(), pair.2)]);         // 如果邻接表不存在则先创建
    }
}

struct DirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,  // key: Vec<(node, weight)>
}

impl DirectedGraph {
    pub fn new() -> Self {
        Self { adjacency_table: HashMap::new() }
    }
}

impl Graph for DirectedGraph {
    fn adjacency_table_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
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

    #[test]
    fn test_neighbours() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5), (String::from("c"), 7)]
        );
    }
}

#[cfg(test)]
mod test_directed_graph {
    use super::DirectedGraph;
    use super::Graph;

    #[test]
    fn test_add_node() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(
            graph.nodes(),
            [&String::from("a"), &String::from("b"), &String::from("c")]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "c", 10));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("b"), &String::from("c"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5)]
        );
    }

    #[test]
    fn test_contains() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(graph.contains("a"), true);
        assert_eq!(graph.contains("b"), true);
        assert_eq!(graph.contains("c"), true);
        assert_eq!(graph.contains("d"), false);
    }
}







