//! 邻接多重表存储结构 - 用于存储无向图

/// 邻接多重表的边节点
#[derive(Debug, Clone)]
pub struct AMLEdge<W> {
    /// 边的两个顶点索引
    pub ivex: usize,
    pub jvex: usize,
    /// 指向依附于 ivex 的下一条边
    pub ilink: Option<usize>,
    /// 指向依附于 jvex 的下一条边
    pub jlink: Option<usize>,
    /// 边的权重
    pub weight: W,
}

/// 邻接多重表的顶点节点
#[derive(Debug, Clone)]
pub struct AMLVertex<T> {
    pub data: T,
    /// 指向第一条依附该顶点的边
    pub first_edge: Option<usize>,
}

/// 邻接多重表（Adjacency Multilist）- 用于存储无向图
///
/// 解决了邻接表存储无向图时，一条边需要两个节点存储，且删除边操作繁琐的问题
#[derive(Debug, Clone)]
pub struct AdjacencyMultilist<T, W> {
    pub vertices: Vec<AMLVertex<T>>,
    pub edges: Vec<Option<AMLEdge<W>>>,
    pub edge_count: usize,
}

impl<T, W> AdjacencyMultilist<T, W>
where
    W: Clone,
{
    pub fn new() -> Self {
        AdjacencyMultilist {
            vertices: Vec::new(),
            edges: Vec::new(),
            edge_count: 0,
        }
    }

    pub fn add_vertex(&mut self, data: T) -> usize {
        let index = self.vertices.len();
        self.vertices.push(AMLVertex {
            data,
            first_edge: None,
        });
        index
    }

    pub fn get_vertex_data(&self, index: usize) -> Option<&T> {
        self.vertices.get(index).map(|v| &v.data)
    }

    /// 添加无向边 (i, j)
    pub fn add_edge(&mut self, i: usize, j: usize, weight: W) {
        if i >= self.vertices.len() || j >= self.vertices.len() {
            panic!("Vertex index out of bounds");
        }
        if i == j {
            panic!("Self loops not supported in this simple implementation");
        }

        // 头插法插入
        let ilink = self.vertices[i].first_edge;
        let jlink = self.vertices[j].first_edge;

        let edge = AMLEdge {
            ivex: i,
            jvex: j,
            ilink,
            jlink,
            weight,
        };

        self.edges.push(Some(edge));
        let edge_idx = self.edges.len() - 1;

        self.vertices[i].first_edge = Some(edge_idx);
        self.vertices[j].first_edge = Some(edge_idx);

        self.edge_count += 1;
    }

    /// 移除无向边 (i, j)
    pub fn remove_edge(&mut self, i: usize, j: usize) {
        if i >= self.vertices.len() || j >= self.vertices.len() {
            return;
        }

        // 查找边索引
        let mut edge_idx_opt = None;
        let mut curr = self.vertices[i].first_edge;
        while let Some(idx) = curr {
            if let Some(edge) = &self.edges[idx] {
                if (edge.ivex == i && edge.jvex == j) || (edge.ivex == j && edge.jvex == i) {
                    edge_idx_opt = Some(idx);
                    break;
                }
                if edge.ivex == i {
                    curr = edge.ilink;
                } else {
                    curr = edge.jlink;
                }
            } else {
                break;
            }
        }

        if let Some(target_idx) = edge_idx_opt {
            self.remove_edge_from_vertex(i, target_idx);
            self.remove_edge_from_vertex(j, target_idx);

            self.edges[target_idx] = None;
            self.edge_count -= 1;
        }
    }

    // 辅助函数：从顶点的链表中移除指定边
    fn remove_edge_from_vertex(&mut self, vertex: usize, target_edge_idx: usize) {
        let mut prev = None;
        let mut curr = self.vertices[vertex].first_edge;
        let mut next_link = None;
        let mut found = false;

        while let Some(idx) = curr {
            let (is_target, i_link, j_link, ivex) = if let Some(edge) = &self.edges[idx] {
                (idx == target_edge_idx, edge.ilink, edge.jlink, edge.ivex)
            } else {
                (false, None, None, 0)
            };

            let link = if ivex == vertex { i_link } else { j_link };

            if is_target {
                next_link = link;
                found = true;
                break;
            }
            prev = Some(idx);
            curr = link;
        }

        if found {
            if let Some(p) = prev {
                if let Some(slot) = self.edges.get_mut(p) {
                    if let Some(prev_edge) = Option::<AMLEdge<W>>::as_mut(slot) {
                        if prev_edge.ivex == vertex {
                            prev_edge.ilink = next_link;
                        } else {
                            prev_edge.jlink = next_link;
                        }
                    }
                }
            } else {
                self.vertices[vertex].first_edge = next_link;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacency_multilist() {
        let mut aml = AdjacencyMultilist::<&str, i32>::new();

        let v0 = aml.add_vertex("A");
        let v1 = aml.add_vertex("B");
        let v2 = aml.add_vertex("C");

        // (A, B) w=1
        aml.add_edge(v0, v1, 1);
        // (B, C) w=2
        aml.add_edge(v1, v2, 2);
        // (A, C) w=3
        aml.add_edge(v0, v2, 3);

        assert_eq!(aml.edge_count, 3);

        // Remove (A, C)
        aml.remove_edge(v0, v2);
        assert_eq!(aml.edge_count, 2);

        // Remove (B, C)
        aml.remove_edge(v1, v2);
        assert_eq!(aml.edge_count, 1);
    }
}
