// use std::{collections::VecDeque, fmt::Display, ops::Deref};

use std::ptr::null_mut;

// type TreeNode<T>=Option<Box<Node<T>>>;
fn main() {
    println!("Hello Tree");
}

struct Tree {
    data: i32,
    left: *mut Tree,
    right: *mut Tree,
    //true表示线索
    l_tag: bool,
    r_tag: bool,
}
fn create_pre_thread(root: *mut Tree) {
    if !root.is_null() {
        let mut pre: *mut Tree = null_mut();
        pre_thread(root, &mut pre);
    }
}
fn pre_thread(current: *mut Tree, pre: &mut *mut Tree) {
    if current.is_null() {
        return;
    }
    visit_pre(current, pre);
    pre_thread(unsafe { (*current).left }, pre);
    pre_thread(unsafe { (*current).right }, pre);
}
fn visit_pre(current: *mut Tree, pre: &mut *mut Tree) {
    if current.is_null() {
        return;
    }
    if unsafe { (*current).left.is_null() } {
        unsafe {
            (*current).left = *pre;
            (*current).l_tag = true;
        }
    }
    unsafe {
        // if !(*pre).is_null() {
        //     if (*(*pre)).right.is_null() {
        //         (*(*pre)).right = current;  // 前驱的后继是当前节点
        //         (*(*pre)).r_tag = true;
        //     }
        // }
        if (*(*pre)).right.is_null() {
            (**pre).right = current;
            (**pre).r_tag = true;
        }
        *pre = current;
    }
}
fn create_in_thread(root: *mut Tree) {
    if root.is_null() {
        return;
    }
    let mut pre_ptr: *mut Tree = null_mut();
    in_thread(root, &mut pre_ptr);

    if !pre_ptr.is_null() {
        unsafe {
            (*pre_ptr).right = null_mut();
            (*pre_ptr).r_tag = true;
        }
    }
}
fn in_thread(current: *mut Tree, pre_ptr: &mut *mut Tree) {
    if current == null_mut() {
        return;
    }
    //左节点
    in_thread(unsafe { (*current).left }, pre_ptr);
    visit(current, pre_ptr);
    in_thread(unsafe { (*current).right }, pre_ptr);
}
fn visit(current: *mut Tree, pre_ptr: &mut *mut Tree) {
    if unsafe { (*current).left } == null_mut() {
        unsafe {
            (*current).left = *pre_ptr;
            (*current).l_tag = true;
        }
    }

    if unsafe { !(*pre_ptr).is_null() && (*(*pre_ptr)).right.is_null() } {
        unsafe {
            (*(*pre_ptr)).right = current;
            (*(*pre_ptr)).r_tag = true;
        }
    }
    *pre_ptr = current;
}
// #[derive(Debug,Default)]
// struct Node<T> {
//     data: T,
//     left: Option<Box<Node<T>>>,
//     right: Option<Box<Node<T>>>,
// }
// impl<T> Node<T> {
//     fn new(data:T)->Self{
//         Self { data, left:None, right:None }
//     }

// }
// //处理节点
//     fn visit<T>(node:&mut Box<Node<T>>)
//     where
//         T:std::fmt::Debug+Display
//     {
//          println!("Visiting node:{}", node.data);
//     }
//     fn level_order_traversal<T>(tree:TreeNode<T>)
//     where T:std::fmt::Debug+Display+Clone
//     {
//         let mut queue=VecDeque::new();
//         //根节点入队
//         if let mut node =tree.unwrap()  {
//             queue.push_back(node);
//         }
//         while !queue.is_empty() {
//             let mut node=queue.pop_front().unwrap();
//             visit(&mut node);
//             if let mut node =node.left.unwrap().clone()  {
//                 queue.push_back(node);
//             }
//             if let node =node.right.unwrap() {
//                 queue.push_back(node);
//             }
//         }

//     }

// impl<T> Deref for Node<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.data
//     }
// }

// impl<T> Drop for Node<T> {
//     fn drop(&mut self) {
//         // 递归释放左子树
//         drop(std::mem::take(&mut self.left));
//         // 递归释放右子树
//         drop(std::mem::take(&mut self.right));
//     }
// }
