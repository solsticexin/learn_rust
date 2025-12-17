//! 图遍历相关的trait和算法

use std::collections::VecDeque;

/// 提供获取图邻接顶点的能力
pub trait GraphNeighbor {
    /// 获取指定顶点的第一个邻接顶点
    ///
    /// # 参数
    /// * `vertex` - 顶点索引
    ///
    /// # 返回值
    /// 返回第一个邻接顶点的索引，如果没有邻接顶点返回None
    fn first_neighbor(&self, vertex: usize) -> Option<usize>;

    /// 获取指定顶点相对于当前邻接顶点的下一个邻接顶点
    ///
    /// # 参数
    /// * `vertex` - 顶点索引
    /// * `current_neighbor` - 当前邻接顶点的索引
    ///
    /// # 返回值
    /// 返回下一个邻接顶点的索引，如果没有更多邻接顶点返回None
    fn next_neighbor(&self, vertex: usize, current_neighbor: usize) -> Option<usize>;
}

/// 顶点访问器，用于在遍历时访问顶点
pub trait VertexVisitor {
    /// 访问一个顶点
    ///
    /// # 参数
    /// * `vertex` - 顶点索引
    fn visit(&mut self, vertex: usize);
}

/// 从指定顶点开始进行广度优先搜索
///
/// # 参数
/// * `graph` - 实现了 GraphNeighbor trait 的图结构
/// * `start` - 起始顶点索引
/// * `visitor` - 实现了 VertexVisitor trait 的访问器
/// * `vertex_count` - 图中顶点的总数
///
/// # 泛型参数
/// * `G` - 图类型，必须实现 GraphNeighbor trait
/// * `V` - 访问器类型，必须实现 VertexVisitor trait
pub fn breadth_first_search<G, V>(graph: &G, start: usize, visitor: &mut V, vertex_count: usize)
where
    G: GraphNeighbor,
    V: VertexVisitor,
{
    // 访问标记数组
    let mut visited = vec![false; vertex_count];
    // 使用队列实现BFS
    let mut queue = VecDeque::new();

    // 访问起始顶点
    visited[start] = true;
    visitor.visit(start);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        // 遍历所有邻接顶点
        let mut neighbor = graph.first_neighbor(current);
        while let Some(next) = neighbor {
            if !visited[next] {
                visited[next] = true;
                visitor.visit(next);
                queue.push_back(next);
            }
            neighbor = graph.next_neighbor(current, next);
        }
    }
}

/// 收集访问顺序的访问器
#[derive(Debug, Default)]
pub struct CollectVisitor {
    /// 访问顺序
    pub order: Vec<usize>,
}

impl VertexVisitor for CollectVisitor {
    fn visit(&mut self, vertex: usize) {
        self.order.push(vertex);
    }
}

/// 打印访问器示例
pub struct PrintVisitor;

impl VertexVisitor for PrintVisitor {
    fn visit(&mut self, vertex: usize) {
        println!("访问顶点: {}", vertex);
    }
}
