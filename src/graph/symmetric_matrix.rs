//! 压缩对称矩阵存储结构

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
