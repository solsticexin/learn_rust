//! 图相关的数据结构和算法实现
//! 包括邻接矩阵存储结构和压缩对称矩阵的方法

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

/// 压缩对称矩阵
/// 用于存储对称矩阵，只保存上三角或下三角部分以节省空间
#[derive(Debug, Clone)]
pub struct SymmetricMatrix {
    /// 矩阵大小 (n x n)
    size: usize,
    /// 压缩存储的元素，只存储下三角部分（包括对角线）
    elements: Vec<i32>,
}

impl SymmetricMatrix {
    /// 创建一个新的对称矩阵
    ///
    /// # 参数
    /// * `size` - 矩阵大小
    ///
    /// # 返回值
    /// 返回一个新的对称矩阵实例
    pub fn new(size: usize) -> Self {
        // 对于 n x n 的矩阵，下三角部分（包括对角线）有 n*(n+1)/2 个元素
        let capacity = size * (size + 1) / 2;
        SymmetricMatrix {
            size,
            elements: vec![0; capacity],
        }
    }

    /// 从二维向量创建对称矩阵
    ///
    /// # 参数
    /// * `matrix` - 二维向量表示的对称矩阵
    ///
    /// # 返回值
    /// 返回一个新的对称矩阵实例
    ///
    /// # Panics
    /// 当输入矩阵不是方阵时会panic
    pub fn from_matrix(matrix: Vec<Vec<i32>>) -> Self {
        let size = matrix.len();

        // 检查是否为方阵
        for row in &matrix {
            if row.len() != size {
                panic!("Input matrix must be square");
            }
        }

        let mut result = SymmetricMatrix::new(size);

        // 只存储下三角部分
        for i in 0..size {
            for j in 0..=i {
                result.set(i, j, matrix[i][j]);
            }
        }

        result
    }

    /// 获取矩阵大小
    pub fn size(&self) -> usize {
        self.size
    }

    /// 将二维索引转换为一维索引
    ///
    /// # 参数
    /// * `row` - 行索引
    /// * `col` - 列索引
    ///
    /// # 返回值
    /// 返回对应的一维索引
    fn to_index(&self, row: usize, col: usize) -> usize {
        if row >= self.size || col >= self.size {
            panic!("Index out of bounds");
        }

        // 确保我们总是访问下三角部分
        let (r, c) = if row >= col { (row, col) } else { (col, row) };

        // 下三角矩阵的一维索引计算公式
        r * (r + 1) / 2 + c
    }

    /// 设置矩阵元素的值
    ///
    /// # 参数
    /// * `row` - 行索引
    /// * `col` - 列索引
    /// * `value` - 要设置的值
    ///
    /// # Panics
    /// 当索引超出范围时会panic
    pub fn set(&mut self, row: usize, col: usize, value: i32) {
        let index = self.to_index(row, col);
        self.elements[index] = value;
    }

    /// 获取矩阵元素的值
    ///
    /// # 参数
    /// * `row` - 行索引
    /// * `col` - 列索引
    ///
    /// # 返回值
    /// 返回指定位置的元素值
    ///
    /// # Panics
    /// 当索引超出范围时会panic
    pub fn get(&self, row: usize, col: usize) -> i32 {
        let index = self.to_index(row, col);
        self.elements[index]
    }

    /// 将压缩矩阵转换为完整的二维向量
    ///
    /// # 返回值
    /// 返回完整的二维向量表示的矩阵
    pub fn to_matrix(&self) -> Vec<Vec<i32>> {
        let mut matrix = Vec::with_capacity(self.size);

        for i in 0..self.size {
            let mut row = Vec::with_capacity(self.size);
            for j in 0..self.size {
                row.push(self.get(i, j));
            }
            matrix.push(row);
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_symmetric_matrix() {
        let mut matrix = SymmetricMatrix::new(3);

        // 设置一些值
        matrix.set(0, 0, 1);
        matrix.set(0, 1, 2);
        matrix.set(0, 2, 3);
        matrix.set(1, 1, 4);
        matrix.set(1, 2, 5);
        matrix.set(2, 2, 6);

        // 验证对称性
        assert_eq!(matrix.get(0, 0), 1);
        assert_eq!(matrix.get(0, 1), 2);
        assert_eq!(matrix.get(1, 0), 2); // 对称
        assert_eq!(matrix.get(0, 2), 3);
        assert_eq!(matrix.get(2, 0), 3); // 对称
        assert_eq!(matrix.get(1, 1), 4);
        assert_eq!(matrix.get(1, 2), 5);
        assert_eq!(matrix.get(2, 1), 5); // 对称
        assert_eq!(matrix.get(2, 2), 6);

        // 测试从矩阵创建
        let original_matrix = vec![vec![1, 2, 3], vec![2, 4, 5], vec![3, 5, 6]];

        let sym_matrix = SymmetricMatrix::from_matrix(original_matrix.clone());
        let restored_matrix = sym_matrix.to_matrix();

        assert_eq!(original_matrix, restored_matrix);
    }
}
