//! 图相关的数据结构和算法实现
//!
//! 本模块提供多种图的存储结构和遍历算法：
//! - 邻接矩阵 (`AdjacencyMatrix`)
//! - 邻接表 (`AdjacencyList`)
//! - 对称矩阵 (`SymmetricMatrix`)
//! - 十字链表 (`OrthogonalList`) - 有向图
//! - 邻接多重表 (`AdjacencyMultilist`) - 无向图
//! - 图遍历相关trait和算法

// 子模块声明
pub mod adjacency_list;
pub mod adjacency_matrix;
pub mod adjacency_multilist;
pub mod orthogonal_list;
pub mod symmetric_matrix;
pub mod traversal;

// 导出主要类型
pub use adjacency_list::AdjacencyList;
pub use adjacency_matrix::AdjacencyMatrix;
pub use adjacency_multilist::{AMLEdge, AMLVertex, AdjacencyMultilist};
pub use orthogonal_list::{OLArc, OLVertex, OrthogonalList};
pub use symmetric_matrix::SymmetricMatrix;
pub use traversal::{
    CollectVisitor, GraphNeighbor, PrintVisitor, VertexVisitor, breadth_first_search,
};
