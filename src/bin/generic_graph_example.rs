use learn_rust::graph::AdjacencyMatrix;

#[derive(Debug, Clone)]
struct City {
    name: String,
    population: u32,
}

#[derive(Debug, Clone)]
struct Road {
    distance: f64,
    max_speed: u32,
}

fn main() {
    println!("=== 泛型图示例 ===");
    
    // 创建一个城市图，顶点存储城市信息，边存储道路信息
    let mut city_graph = AdjacencyMatrix::<City, Road>::new(3);
    
    // 设置城市数据
    city_graph.set_vertex_data(0, City {
        name: "北京".to_string(),
        population: 21540000,
    });
    city_graph.set_vertex_data(1, City {
        name: "上海".to_string(),
        population: 24280000,
    });
    city_graph.set_vertex_data(2, City {
        name: "广州".to_string(),
        population: 15300000,
    });
    
    // 添加道路连接
    city_graph.add_edge(0, 1, Some(Road {
        distance: 1066.0,
        max_speed: 120,
    }));
    city_graph.add_edge(1, 2, Some(Road {
        distance: 1412.0,
        max_speed: 110,
    }));
    city_graph.add_edge(0, 2, Some(Road {
        distance: 1960.0,
        max_speed: 100,
    }));
    
    println!("城市图信息:");
    println!("顶点数: {}, 边数: {}", city_graph.vertices(), city_graph.edges());
    
    println!("\n城市详情:");
    for i in 0..city_graph.vertices() {
        if let Some(city) = city_graph.get_vertex_data(i) {
            println!("  {}: {} (人口: {})", i, city.name, city.population);
        }
    }
    
    println!("\n道路连接:");
    for i in 0..city_graph.vertices() {
        for j in 0..city_graph.vertices() {
            if let Some(road) = city_graph.get_edge(i, j) {
                if let (Some(from_city), Some(to_city)) = 
                    (city_graph.get_vertex_data(i), city_graph.get_vertex_data(j)) {
                    println!("  {} -> {}: 距离 {:.1} 公里, 最高限速 {} km/h", 
                             from_city.name, to_city.name, road.distance, road.max_speed);
                }
            }
        }
    }
    
    println!("\n=== 整数权重图示例 ===");
    
    // 创建一个简单的整数权重图
    let mut int_graph = AdjacencyMatrix::<String, i32>::new(3);
    
    // 设置顶点名称
    int_graph.set_vertex_data(0, "A".to_string());
    int_graph.set_vertex_data(1, "B".to_string());
    int_graph.set_vertex_data(2, "C".to_string());
    
    // 添加带权重的边
    int_graph.add_edge(0, 1, Some(10));
    int_graph.add_edge(1, 2, Some(20));
    int_graph.add_edge(0, 2, Some(30));
    
    println!("整数权重图:");
    for i in 0..int_graph.vertices() {
        for j in 0..int_graph.vertices() {
            if let Some(weight) = int_graph.get_edge(i, j) {
                if let (Some(from_name), Some(to_name)) = 
                    (int_graph.get_vertex_data(i), int_graph.get_vertex_data(j)) {
                    println!("  {} -> {}: 权重 {}", from_name, to_name, weight);
                }
            }
        }
    }
}