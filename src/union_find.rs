pub struct UnionFind(Vec<isize>);

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self(vec![-1; size])
    }
    ///寻找根节点
    pub fn find(&mut self, mut x: isize) -> Result<isize, &'static str> {
        if x < 0 || x >= self.0.len() as isize {
            return Err("index out of bounds");
        }
        let mut root = x;
        //寻找元素所在集合的根结点
        while self.0[root as usize] >= 0 {
            root = self.0[root as usize];
        }
        //压缩路径
        while x != root {
            let next = self.0[x as usize];
            self.0[x as usize] = root;
            x = next;
        }
        Ok(root)
    }
    pub fn union(&mut self, x: isize, y: isize) -> Result<(), &str> {
        //找到根节点
        let root1 = self.find(x)?;
        let root2 = self.find(y)?;
        if root1 == root2 {
            return Ok(());
        }
        //小数并到大树
        if self.0[root1 as usize].abs() > self.0[root2 as usize].abs() {
            self.0[root1 as usize] += self.0[root2 as usize];
            self.0[root2 as usize] = root1;
        } else {
            self.0[root2 as usize] += self.0[root1 as usize];
            self.0[root1 as usize] = root2;
        }
        Ok(())
    }
}
