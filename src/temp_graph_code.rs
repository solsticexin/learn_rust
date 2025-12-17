/// 十字链表（Orthogonal List）- 用于存储有向图
///
/// 能够方便地求得顶点的出度和入度
/// 解决了邻接矩阵空间浪费和邻接表无法同时方便查找入边和出边的问题
#[derive(Debug, Clone)]
pub struct OLArc<W> {
    /// 弧尾（起点）顶点下标
    pub tail_vex: usize,
    /// 弧头（终点）顶点下标
    pub head_vex: usize,
    /// 指向弧头相同的下一条弧（入边表指针）
    pub head_link: Option<usize>,
    /// 指向弧尾相同的下一条弧（出边表指针）
    pub tail_link: Option<usize>,
    /// 弧的权重
    pub weight: W,
}

#[derive(Debug, Clone)]
pub struct OLVertex<T> {
    /// 顶点数据
    pub data: T,
    /// 指向第一条入弧的下标
    pub first_in: Option<usize>,
    /// 指向第一条出弧的下标
    pub first_out: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct OrthogonalList<T, W> {
    /// 顶点列表
    pub vertices: Vec<OLVertex<T>>,
    /// 弧的存储池（Arena），使用Option以支持删除（虽然本例未实现完全的回收机制）
    pub arcs: Vec<Option<OLArc<W>>>,
    /// 空闲弧的链表头，用于重用被删除的位置
    free_arc_head: Option<usize>,
    /// 边的数量
    edge_count: usize,
}

impl<T, W> OrthogonalList<T, W>
where
    W: Clone,
{
    /// 创建一个新的十字链表图
    pub fn new() -> Self {
        OrthogonalList {
            vertices: Vec::new(),
            arcs: Vec::new(),
            free_arc_head: None,
            edge_count: 0,
        }
    }

    /// 添加顶点
    pub fn add_vertex(&mut self, data: T) -> usize {
        let index = self.vertices.len();
        self.vertices.push(OLVertex {
            data,
            first_in: None,
            first_out: None,
        });
        index
    }

    /// 获取顶点数据
    pub fn get_vertex_data(&self, index: usize) -> Option<&T> {
        self.vertices.get(index).map(|v| &v.data)
    }

    /// 分配一个新的弧槽位
    fn alloc_arc(&mut self, arc: OLArc<W>) -> usize {
        if let Some(idx) = self.free_arc_head {
            // 如果有空闲槽位，重用之
            // 此时 arcs[idx] 实际上没有存储有效数据，我们需要知道下一个空闲位置在哪
            // 这里的简单实现假设删除时我们维护了 free_arc_head
            // 为了简化，我们暂时不实现复杂的空闲链表逻辑，而是简单地寻找None或者push
            // 真实的FreeList实现需要像 slotmap 那样

            // 修正：为了KISS，我们简单地push新元素，只有当完全实现删除回收逻辑时才考虑重用。
            // 下面的实现主要关注结构正确性。
            self.arcs.push(Some(arc));
            self.arcs.len() - 1
        } else {
            self.arcs.push(Some(arc));
            self.arcs.len() - 1
        }
    }

    /// 添加一条有向边
    pub fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        if from >= self.vertices.len() || to >= self.vertices.len() {
            panic!("Vertex index out of bounds");
        }

        // 创建新弧
        // 头插法插入到 tail_vex 的出边表
        let tail_link = self.vertices[from].first_out;
        // 头插法插入到 head_vex 的入边表
        let head_link = self.vertices[to].first_in;

        let arc = OLArc {
            tail_vex: from,
            head_vex: to,
            head_link,
            tail_link,
            weight,
        };

        let arc_idx = self.alloc_arc(arc);

        // 更新顶点的指针
        self.vertices[from].first_out = Some(arc_idx);
        self.vertices[to].first_in = Some(arc_idx);

        self.edge_count += 1;
    }

    /// 获取边的权重
    pub fn get_edge(&self, from: usize, to: usize) -> Option<&W> {
        if from >= self.vertices.len() || to >= self.vertices.len() {
            return None;
        }

        // 遍历 from 的出边表寻找
        let mut curr = self.vertices[from].first_out;
        while let Some(idx) = curr {
            if let Some(arc) = &self.arcs[idx] {
                if arc.head_vex == to {
                    return Some(&arc.weight);
                }
                curr = arc.tail_link;
            } else {
                // Should not happen in valid list
                break;
            }
        }
        None
    }

    /// 移除一条有向边
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        if from >= self.vertices.len() || to >= self.vertices.len() {
            return;
        }

        // 1. 从 from 的出边表中移除
        let mut prev = None;
        let mut curr = self.vertices[from].first_out;
        while let Some(idx) = curr {
            if let Some(arc) = &self.arcs[idx] {
                if arc.head_vex == to {
                    // 找到边，断开链接
                    if let Some(p) = prev {
                        // 在中间或末尾，前驱指向后继
                        if let Some(prev_arc) = self.arcs[p].as_mut() {
                            prev_arc.tail_link = arc.tail_link;
                        }
                    } else {
                        // 是第一条边，修改头指针
                        self.vertices[from].first_out = arc.tail_link;
                    }
                    break;
                }
                prev = Some(idx);
                curr = arc.tail_link;
            } else {
                break;
            }
        }

        // 2. 从 to 的入边表中移除
        // 注意：这里我们必须再次查找该边的索引，因为上面的循环中其实拿到过，但为了逻辑清晰再次查找也无妨，或者优化下逻辑
        // 实际上，我们应该在找到该边时记录下索引 idx_to_remove

        // 重新完整的查找和移除逻辑，确保在两个链表中都正确移除
        // 为了简单起见，我将重写逻辑：先找到边的索引，然后分别在两个链表中执行删除

        let mut arc_idx_opt = None;

        // Find arc index
        let mut curr = self.vertices[from].first_out;
        while let Some(idx) = curr {
            if let Some(arc) = &self.arcs[idx] {
                if arc.head_vex == to {
                    arc_idx_opt = Some(idx);
                    break;
                }
                curr = arc.tail_link;
            } else {
                break;
            }
        }

        if let Some(target_idx) = arc_idx_opt {
            // 移除 from 的出边链
            let mut prev = None;
            let mut curr = self.vertices[from].first_out;
            while let Some(idx) = curr {
                if idx == target_idx {
                    if let Some(p) = prev {
                        if let Some(prev_arc) = self.arcs[p].as_mut() {
                            prev_arc.tail_link = self.arcs[idx].as_ref().unwrap().tail_link;
                        }
                    } else {
                        self.vertices[from].first_out = self.arcs[idx].as_ref().unwrap().tail_link;
                    }
                    break;
                }
                prev = Some(idx);
                if let Some(arc) = &self.arcs[idx] {
                    curr = arc.tail_link;
                } else {
                    break;
                }
            }

            // 移除 to 的入边链
            let mut prev = None;
            let mut curr = self.vertices[to].first_in;
            while let Some(idx) = curr {
                if idx == target_idx {
                    if let Some(p) = prev {
                        if let Some(prev_arc) = self.arcs[p].as_mut() {
                            prev_arc.head_link = self.arcs[idx].as_ref().unwrap().head_link;
                        }
                    } else {
                        self.vertices[to].first_in = self.arcs[idx].as_ref().unwrap().head_link;
                    }
                    break;
                }
                prev = Some(idx);
                if let Some(arc) = &self.arcs[idx] {
                    curr = arc.head_link;
                } else {
                    break;
                }
            }

            // 回收空间
            // 简单标记为 None，不做复杂的空闲链表维护以保持KISS
            self.arcs[target_idx] = None;
            self.edge_count -= 1;
        }
    }
}

/// 邻接多重表（Adjacency Multilist）- 用于存储无向图
///
/// 解决了邻接表存储无向图时，一条边需要两个节点存储，且删除边操作繁琐的问题
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

#[derive(Debug, Clone)]
pub struct AMLVertex<T> {
    pub data: T,
    /// 指向第一条依附该顶点的边
    pub first_edge: Option<usize>,
}

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

        while let Some(idx) = curr {
            if idx == target_edge_idx {
                // 确定下一条边是 ilink 还是 jlink
                let next_link = {
                    let edge = self.edges[idx].as_ref().unwrap();
                    if edge.ivex == vertex {
                        edge.ilink
                    } else {
                        edge.jlink
                    }
                };

                if let Some(p) = prev {
                    // 修改前驱的指针
                    if let Some(prev_edge) = self.edges[p].as_mut() {
                        if prev_edge.ivex == vertex {
                            prev_edge.ilink = next_link;
                        } else {
                            prev_edge.jlink = next_link;
                        }
                    }
                } else {
                    // 修改头指针
                    self.vertices[vertex].first_edge = next_link;
                }
                break;
            }

            prev = Some(idx);
            if let Some(edge) = &self.edges[idx] {
                if edge.ivex == vertex {
                    curr = edge.ilink;
                } else {
                    curr = edge.jlink;
                }
            } else {
                break;
            }
        }
    }
}
