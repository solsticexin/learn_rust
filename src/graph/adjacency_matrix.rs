//! 邻接矩阵存储结构

use super::traversal::GraphNeighbor;

/// 图的邻接矩阵存储结构
///
/// # 类型参数
/// * `T` - 顶点存储的数据类型
/// * `W` - 边的权重类型
#[derive(Debug, Clone)]
pub struct AdjacencyMatrix<T, W> {
    /// 顶点数量
    vertices: usize,
    /// 边的数量
    edges: usize,
    /// 顶点数据
    vertex_data: Vec<Option<T>>,
    /// 邻接矩阵，使用二维向量存储
    matrix: Vec<Vec<Option<W>>>,
}

impl<T, W> AdjacencyMatrix<T, W>
where
    W: Clone,
{
    /// 创建一个新的邻接矩阵
    ///
    /// # 参数
    /// * `vertices` - 顶点数量
    ///
    /// # 返回值
    /// 返回一个新的邻接矩阵实例
    pub fn new(vertices: usize) -> Self {
        let mut matrix = Vec::with_capacity(vertices);
        for _ in 0..vertices {
            let mut row = Vec::with_capacity(vertices);
            for _ in 0..vertices {
                row.push(None);
            }
            matrix.push(row);
        }

        let mut vertex_data = Vec::with_capacity(vertices);
        for _ in 0..vertices {
            vertex_data.push(None);
        }

        AdjacencyMatrix {
            vertices,
            edges: 0,
            vertex_data,
            matrix,
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
    ///
    /// # 参数
    /// * `vertex` - 顶点索引
    /// * `data` - 要存储的数据
    ///
    /// # Panics
    /// 当顶点索引超出范围时会panic
    pub fn set_vertex_data(&mut self, vertex: usize, data: T) {
        if vertex >= self.vertices {
            panic!("Vertex index out of bounds");
        }
        self.vertex_data[vertex] = Some(data);
    }

    /// 获取顶点的数据
    ///
    /// # 参数
    /// * `vertex` - 顶点索引
    ///
    /// # 返回值
    /// 返回顶点的数据，如果顶点没有数据则返回None
    ///
    /// # Panics
    /// 当顶点索引超出范围时会panic
    pub fn get_vertex_data(&self, vertex: usize) -> Option<&T> {
        if vertex >= self.vertices {
            panic!("Vertex index out of bounds");
        }
        self.vertex_data[vertex].as_ref()
    }

    /// 在两个顶点之间添加一条边
    ///
    /// # 参数
    /// * `from` - 起始顶点
    /// * `to` - 终止顶点
    /// * `weight` - 边的权重，None表示无边，Some(value)表示有权重的边
    ///
    /// # Panics
    /// 当顶点索引超出范围时会panic
    pub fn add_edge(&mut self, from: usize, to: usize, weight: Option<W>) {
        if from >= self.vertices || to >= self.vertices {
            panic!("Vertex index out of bounds");
        }

        if self.matrix[from][to].is_none() && weight.is_some() {
            self.edges += 1;
        } else if self.matrix[from][to].is_some() && weight.is_none() {
            self.edges -= 1;
        }

        self.matrix[from][to] = weight;
        // 如果是无向图，同时设置对称位置
        // self.matrix[to][from] = weight;
    }

    /// 获取两个顶点之间的边的权重
    ///
    /// # 参数
    /// * `from` - 起始顶点
    /// * `to` - 终止顶点
    ///
    /// # 返回值
    /// 返回边的权重，None表示无边
    ///
    /// # Panics
    /// 当顶点索引超出范围时会panic
    pub fn get_edge(&self, from: usize, to: usize) -> Option<&W> {
        if from >= self.vertices || to >= self.vertices {
            panic!("Vertex index out of bounds");
        }
        self.matrix[from][to].as_ref()
    }

    /// 移除两个顶点之间的边
    ///
    /// # 参数
    /// * `from` - 起始顶点
    /// * `to` - 终止顶点
    ///
    /// # Panics
    /// 当顶点索引超出范围时会panic
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        if from >= self.vertices || to >= self.vertices {
            panic!("Vertex index out of bounds");
        }

        if self.matrix[from][to].is_some() {
            self.edges -= 1;
        }
        self.matrix[from][to] = None;
    }
}

// 为 AdjacencyMatrix 实现 GraphNeighbor trait
impl<T, W> GraphNeighbor for AdjacencyMatrix<T, W>
where
    W: Clone,
{
    fn first_neighbor(&self, vertex: usize) -> Option<usize> {
        if vertex >= self.vertices {
            return None;
        }

        // 从第0个顶点开始寻找第一个邻接顶点
        for i in 0..self.vertices {
            if self.matrix[vertex][i].is_some() {
                return Some(i);
            }
        }
        None
    }

    fn next_neighbor(&self, vertex: usize, current_neighbor: usize) -> Option<usize> {
        if vertex >= self.vertices || current_neighbor >= self.vertices {
            return None;
        }

        // 从current_neighbor的下一个位置开始寻找
        for i in (current_neighbor + 1)..self.vertices {
            if self.matrix[vertex][i].is_some() {
                return Some(i);
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
    fn test_adjacency_matrix() {
        let mut graph = AdjacencyMatrix::<String, i32>::new(4);

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
        assert_eq!(graph.get_vertex_data(2), Some(&"Node C".to_string()));
        assert_eq!(graph.get_vertex_data(3), Some(&"Node D".to_string()));

        graph.add_edge(0, 1, Some(5));
        graph.add_edge(1, 2, Some(3));
        graph.add_edge(2, 3, Some(7));

        assert_eq!(graph.edges(), 3);
        assert_eq!(graph.get_edge(0, 1), Some(&5));
        assert_eq!(graph.get_edge(1, 2), Some(&3));
        assert_eq!(graph.get_edge(2, 3), Some(&7));
        assert_eq!(graph.get_edge(0, 2), None);

        graph.remove_edge(1, 2);
        assert_eq!(graph.edges(), 2);
        assert_eq!(graph.get_edge(1, 2), None);
    }

    #[test]
    fn test_bfs_adjacency_matrix() {
        // 创建一个简单的有向图用于测试
        // 图结构:
        //     0 → 1 → 3
        //     ↓   ↓
        //     2 → 4
        let mut graph = AdjacencyMatrix::<String, i32>::new(5);

        // 添加边
        graph.add_edge(0, 1, Some(1));
        graph.add_edge(0, 2, Some(1));
        graph.add_edge(1, 3, Some(1));
        graph.add_edge(1, 4, Some(1));
        graph.add_edge(2, 4, Some(1));

        // 使用CollectVisitor收集访问顺序
        let mut visitor = CollectVisitor::default();
        breadth_first_search(&graph, 0, &mut visitor, 5);

        // BFS从0开始的访问顺序应该是: 0, 1, 2, 3, 4
        // 或者 0, 2, 1, 4, 3 (取决于邻接顺序)
        // 邻接矩阵按索引顺序返回邻接顶点，所以应该是 0, 1, 2, 3, 4
        assert_eq!(visitor.order, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_graph_neighbor_trait() {
        // 测试 GraphNeighbor trait 的基本功能
        let mut graph = AdjacencyMatrix::<String, i32>::new(4);
        graph.add_edge(0, 1, Some(1));
        graph.add_edge(0, 2, Some(1));
        graph.add_edge(1, 3, Some(1));

        // 测试 first_neighbor
        assert_eq!(graph.first_neighbor(0), Some(1));
        assert_eq!(graph.first_neighbor(1), Some(3));
        assert_eq!(graph.first_neighbor(2), None);
        assert_eq!(graph.first_neighbor(3), None);

        // 测试 next_neighbor
        assert_eq!(graph.next_neighbor(0, 1), Some(2));
        assert_eq!(graph.next_neighbor(0, 2), None);
        assert_eq!(graph.next_neighbor(1, 3), None);
    }
}
