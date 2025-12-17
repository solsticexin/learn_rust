//! 十字链表存储结构 - 用于存储有向图

/// 十字链表的弧节点
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

/// 十字链表的顶点节点
#[derive(Debug, Clone)]
pub struct OLVertex<T> {
    /// 顶点数据
    pub data: T,
    /// 指向第一条入弧的下标
    pub first_in: Option<usize>,
    /// 指向第一条出弧的下标
    pub first_out: Option<usize>,
}

/// 十字链表（Orthogonal List）- 用于存储有向图
///
/// 能够方便地求得顶点的出度和入度
/// 解决了邻接矩阵空间浪费和邻接表无法同时方便查找入边和出边的问题
#[derive(Debug, Clone)]
pub struct OrthogonalList<T, W> {
    /// 顶点列表
    pub vertices: Vec<OLVertex<T>>,
    /// 弧的存储池（Arena），使用Option以支持删除
    pub arcs: Vec<Option<OLArc<W>>>,
    /// 空闲弧的链表头，用于重用被删除的位置
    free_arc_head: Option<usize>,
    /// 边的数量
    pub(crate) edge_count: usize,
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
        if let Some(_idx) = self.free_arc_head {
            // 简化实现：直接push新元素
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
        let tail_link = self.vertices[from].first_out;
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

        // 从出边表中移除
        let mut prev = None;
        let mut curr = self.vertices[from].first_out;
        let mut target_idx_opt = None;
        let mut next_link = None;

        while let Some(idx) = curr {
            let (is_target, link) = if let Some(arc) = &self.arcs[idx] {
                (arc.head_vex == to, arc.tail_link)
            } else {
                (false, None)
            };

            if is_target {
                target_idx_opt = Some(idx);
                next_link = link;
                break;
            }
            prev = Some(idx);
            curr = link;
        }

        if let Some(target_idx) = target_idx_opt {
            if let Some(p) = prev {
                if let Some(slot) = self.arcs.get_mut(p) {
                    if let Some(prev_arc) = Option::<OLArc<W>>::as_mut(slot) {
                        prev_arc.tail_link = next_link;
                    }
                }
            } else {
                self.vertices[from].first_out = next_link;
            }

            // 从入边表中移除
            let mut prev = None;
            let mut curr = self.vertices[to].first_in;
            let mut found_in_list = false;
            let mut next_link_in = None;

            while let Some(idx) = curr {
                let (is_target, link) = if let Some(arc) = &self.arcs[idx] {
                    (idx == target_idx, arc.head_link)
                } else {
                    (false, None)
                };

                if is_target {
                    found_in_list = true;
                    next_link_in = link;
                    break;
                }
                prev = Some(idx);
                curr = link;
            }

            if found_in_list {
                if let Some(p) = prev {
                    if let Some(slot) = self.arcs.get_mut(p) {
                        if let Some(prev_arc) = Option::<OLArc<W>>::as_mut(slot) {
                            prev_arc.head_link = next_link_in;
                        }
                    }
                } else {
                    self.vertices[to].first_in = next_link_in;
                }
            }

            // 标记槽位为空
            self.arcs[target_idx] = None;
            self.edge_count -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orthogonal_list() {
        let mut ol = OrthogonalList::<&str, i32>::new();

        let v0 = ol.add_vertex("V0");
        let v1 = ol.add_vertex("V1");
        let v2 = ol.add_vertex("V2");
        let v3 = ol.add_vertex("V3");

        // V0 -> V1, weight 10
        ol.add_edge(v0, v1, 10);
        // V0 -> V2, weight 20
        ol.add_edge(v0, v2, 20);
        // V2 -> V3, weight 30
        ol.add_edge(v2, v3, 30);
        // V3 -> V0, weight 40
        ol.add_edge(v3, v0, 40);

        assert_eq!(ol.edge_count, 4);
        assert_eq!(ol.get_edge(v0, v1), Some(&10));
        assert_eq!(ol.get_edge(v3, v0), Some(&40));
        assert_eq!(ol.get_edge(v1, v2), None);

        // Remove edge V0 -> V2
        ol.remove_edge(v0, v2);
        assert_eq!(ol.edge_count, 3);
        assert_eq!(ol.get_edge(v0, v2), None);

        // Check linked list integrity
        assert_eq!(ol.get_edge(v0, v1), Some(&10));
    }
}
