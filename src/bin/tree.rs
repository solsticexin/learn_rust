use std::{collections::VecDeque, ops::Deref};

type TreeNode<T>=Option<Box<Node<T>>>;
fn main() {
    println!("Hello Tree");
}

struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(data:T)->Self{
        Self { data, left:None, right:None }
    }
   
}
//处理节点
    fn visit<T>(node:&mut Box<Node<T>>){
         println!("Visiting node:{}", node.data);
    } 
    fn level_order_traversal<T>(tree:TreeNode<T>){
        let mut queue=VecDeque::new();
        //根节点入队
        if let mut node =tree.unwrap()  {
            queue.push_back(&mut node);
        }
        while !queue.is_empty() {
            let node=queue.pop_front().unwrap();
            visit(node);
            if let node = &mut node.left.unwrap()  {
                queue.push_back(node);
            }
            if let node =&mut node.right.unwrap() {
                queue.push_back(node);
            }
        }
        
    }

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        // 递归释放左子树
        drop(std::mem::take(&mut self.left));
        // 递归释放右子树
        drop(std::mem::take(&mut self.right));
    }
}
