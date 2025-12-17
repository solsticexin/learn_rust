//! 邻接表存储结构

use super::traversal::GraphNeighbor;

/// 图的邻接表存储结构
///
/// # 类型参数
/// * `T` - 顶点存储的数据类型
/// * `W` - 边的权重类型
#[derive(Debug, Clone)]
pub struct AdjacencyList<T, W> {
    /// 顶点数量
    vertices: usize,
    /// 边的数量
    edges: usize,
    /// 顶点数据
    vertex_data: Vec<Option<T>>,
    /// 邻接表，使用向量的向量存储，内部存储 (目标顶点, 权重)
    adj: Vec<Vec<(usize, W)>>,
}

impl<T, W> AdjacencyList<T, W>
where
    W: Clone + PartialEq,
{
    /// 创建一个新的邻接表
    ///
    /// # 参数
    /// * `vertices` - 顶点数量
    pub fn new(vertices: usize) -> Self {
        let mut adj = Vec::with_capacity(vertices);
        for _ in 0..vertices {
            adj.push(Vec::new());
        }

        let mut vertex_data = Vec::with_capacity(vertices);
        for _ in 0..vertices {
            vertex_data.push(None);
        }

        AdjacencyList {
            vertices,
            edges: 0,
            vertex_data,
            adj,
        }
    }

    /// 获取顶点数量
    pub fn vertices(&self) -> usize {
        self.vertices
    }

    /// 获取边的数量
    pub fn edges(&self) -> usize {
        self.edges
    }

    /// 设置顶点的数据
    pub fn set_vertex_data(&mut self, vertex: usize, data: T) {
        if vertex >= self.vertices {
            panic!("Vertex index out of bounds");
        }
        self.vertex_data[vertex] = Some(data);
    }

    /// 获取顶点的数据
    pub fn get_vertex_data(&self, vertex: usize) -> Option<&T> {
        if vertex >= self.vertices {
            panic!("Vertex index out of bounds");
        }
        self.vertex_data[vertex].as_ref()
    }

    /// 添加一条边
    ///
    /// # 参数
    /// * `from` - 起始顶点
    /// * `to` - 终止顶点
    /// * `weight` - 边的权重
    pub fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        if from >= self.vertices || to >= self.vertices {
            panic!("Vertex index out of bounds");
        }

        // 检查边是否已存在，如果存在则更新权重
        if let Some(edge) = self.adj[from].iter_mut().find(|(v, _)| *v == to) {
            edge.1 = weight;
            return;
        }

        self.adj[from].push((to, weight));
        self.edges += 1;
    }

    /// 获取两个顶点之间的边的权重
    pub fn get_edge(&self, from: usize, to: usize) -> Option<&W> {
        if from >= self.vertices || to >= self.vertices {
            panic!("Vertex index out of bounds");
        }

        self.adj[from]
            .iter()
            .find(|(v, _)| *v == to)
            .map(|(_, w)| w)
    }

    /// 移除两个顶点之间的边
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        if from >= self.vertices || to >= self.vertices {
            panic!("Vertex index out of bounds");
        }

        if let Some(idx) = self.adj[from].iter().position(|(v, _)| *v == to) {
            self.adj[from].remove(idx);
            self.edges -= 1;
        }
    }
}

// 为 AdjacencyList 实现 GraphNeighbor trait
impl<T, W> GraphNeighbor for AdjacencyList<T, W>
where
    W: Clone + PartialEq,
{
    fn first_neighbor(&self, vertex: usize) -> Option<usize> {
        if vertex >= self.vertices {
            return None;
        }

        // 邻接表的第一个邻接顶点就是列表的第一个元素
        self.adj[vertex].first().map(|(v, _)| *v)
    }

    fn next_neighbor(&self, vertex: usize, current_neighbor: usize) -> Option<usize> {
        if vertex >= self.vertices {
            return None;
        }

        // 在邻接表中找到current_neighbor的位置，然后返回下一个
        let adj_list = &self.adj[vertex];
        for i in 0..adj_list.len() {
            if adj_list[i].0 == current_neighbor {
                // 找到了当前邻接顶点，返回下一个
                if i + 1 < adj_list.len() {
                    return Some(adj_list[i + 1].0);
                }
                return None;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::traversal::{CollectVisitor, breadth_first_search};

    #[test]
    fn test_adjacency_list() {
        let mut graph = AdjacencyList::<String, i32>::new(4);

        assert_eq!(graph.vertices(), 4);
        assert_eq!(graph.edges(), 0);

        // 设置顶点数据
        graph.set_vertex_data(0, "Node A".to_string());
        graph.set_vertex_data(1, "Node B".to_string());
        graph.set_vertex_data(2, "Node C".to_string());
        graph.set_vertex_data(3, "Node D".to_string());

        // 验证顶点数据
        assert_eq!(graph.get_vertex_data(0), Some(&"Node A".to_string()));
        assert_eq!(graph.get_vertex_data(1), Some(&"Node B".to_string()));

        graph.add_edge(0, 1, 5);
        graph.add_edge(1, 2, 3);
        graph.add_edge(2, 3, 7);

        assert_eq!(graph.edges(), 3);
        assert_eq!(graph.get_edge(0, 1), Some(&5));
        assert_eq!(graph.get_edge(1, 2), Some(&3));
        assert_eq!(graph.get_edge(2, 3), Some(&7));
        assert_eq!(graph.get_edge(0, 2), None);

        // 更新边权重
        graph.add_edge(0, 1, 10);
        assert_eq!(graph.edges(), 3); // 边数不变
        assert_eq!(graph.get_edge(0, 1), Some(&10));

        graph.remove_edge(1, 2);
        assert_eq!(graph.edges(), 2);
        assert_eq!(graph.get_edge(1, 2), None);
    }

    #[test]
    fn test_bfs_adjacency_list() {
        // 创建相同的有向图
        let mut graph = AdjacencyList::<String, i32>::new(5);

        // 添加边（注意：邻接表的顺序取决于添加顺序）
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(1, 4, 1);
        graph.add_edge(2, 4, 1);

        // 使用CollectVisitor收集访问顺序
        let mut visitor = CollectVisitor::default();
        breadth_first_search(&graph, 0, &mut visitor, 5);

        // BFS从0开始应该访问所有顶点
        assert_eq!(visitor.order.len(), 5);
        assert_eq!(visitor.order[0], 0); // 第一个访问的必定是起点

        // 检查第二层（1和2）在第三层（3和4）之前被访问
        let pos_1 = visitor.order.iter().position(|&x| x == 1).unwrap();
        let pos_2 = visitor.order.iter().position(|&x| x == 2).unwrap();
        let pos_3 = visitor.order.iter().position(|&x| x == 3).unwrap();
        let pos_4 = visitor.order.iter().position(|&x| x == 4).unwrap();

        assert!(pos_1 < pos_3);
        assert!(pos_1 < pos_4 || pos_2 < pos_4); // 至少一个第二层顶点在4之前
    }
}
