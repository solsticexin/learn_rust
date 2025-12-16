# Learn Rust

这是一个用于学习 Rust 编程语言的项目，包含了各种数据结构和算法的实现。

## 项目结构

```
src/
├── bin/              # 可执行文件
│   ├── 13.1.rs
│   ├── guessing_game.rs
│   ├── minigrep.rs
│   └── stack_application.rs
├── lib.rs            # 库文件入口
├── main.rs           # 主程序
├── tree.rs           # 树数据结构
├── union_find.rs     # 并查集数据结构
└── graph.rs          # 图数据结构（邻接矩阵和压缩对称矩阵）
```

## 模块介绍

### graph 模块
该模块实现了图的相关数据结构：

1. `AdjacencyMatrix<T, W>` - 图的邻接矩阵存储结构
   - 泛型结构，支持存储顶点数据(T)和边权重(W)
   - 支持有向图和无向图
   - 可以存储带权图
   - 提供添加边、删除边、查询边等操作
   - 支持设置和获取顶点数据

2. `SymmetricMatrix` - 压缩对称矩阵
   - 用于存储对称矩阵，节省存储空间
   - 只存储矩阵的下三角部分（包括对角线）
   - 提供完整的矩阵访问接口

### tree 模块
(待补充)

### union_find 模块
(待补充)

## 运行示例

```bash
# 运行基础图数据结构示例
cargo run --bin graph_example

# 运行泛型图示例
cargo run --bin generic_graph_example

# 运行测试
cargo test
```