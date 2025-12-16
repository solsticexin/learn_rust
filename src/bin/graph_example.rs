use learn_rust::graph::{AdjacencyMatrix, SymmetricMatrix};

fn main() {
    println!("=== 图的邻接矩阵示例 ===");
    
    // 创建一个包含4个顶点的图，顶点存储字符串数据，边存储整数权重
    let mut graph = AdjacencyMatrix::<String, i32>::new(4);
    
    // 设置顶点数据
    graph.set_vertex_data(0, "节点A".to_string());
    graph.set_vertex_data(1, "节点B".to_string());
    graph.set_vertex_data(2, "节点C".to_string());
    graph.set_vertex_data(3, "节点D".to_string());
    
    // 添加一些边
    graph.add_edge(0, 1, Some(5));
    graph.add_edge(1, 2, Some(3));
    graph.add_edge(2, 3, Some(7));
    graph.add_edge(0, 3, Some(2));
    
    println!("图的顶点数: {}", graph.vertices());
    println!("图的边数: {}", graph.edges());
    
    println!("顶点数据:");
    for i in 0..graph.vertices() {
        if let Some(data) = graph.get_vertex_data(i) {
            println!("  顶点 {}: {}", i, data);
        }
    }
    
    println!("边的详细信息:");
    for i in 0..graph.vertices() {
        for j in 0..graph.vertices() {
            if let Some(weight) = graph.get_edge(i, j) {
                println!("  顶点 {} -> 顶点 {}: 权重 {}", i, j, weight);
            }
        }
    }
    
    println!("\n=== 压缩对称矩阵示例 ===");
    
    // 创建一个对称矩阵
    let mut sym_matrix = SymmetricMatrix::new(3);
    
    // 设置矩阵元素
    sym_matrix.set(0, 0, 1);
    sym_matrix.set(0, 1, 2);
    sym_matrix.set(0, 2, 3);
    sym_matrix.set(1, 1, 4);
    sym_matrix.set(1, 2, 5);
    sym_matrix.set(2, 2, 6);
    
    println!("压缩对称矩阵:");
    for i in 0..sym_matrix.size() {
        for j in 0..sym_matrix.size() {
            print!("{} ", sym_matrix.get(i, j));
        }
        println!();
    }
    
    // 转换为完整矩阵并打印
    println!("\n完整矩阵表示:");
    let full_matrix = sym_matrix.to_matrix();
    for row in full_matrix {
        println!("{:?}", row);
    }
}