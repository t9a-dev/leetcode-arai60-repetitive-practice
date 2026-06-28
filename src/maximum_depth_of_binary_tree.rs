use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct MaximumDepthOfBinaryTree {}
impl MaximumDepthOfBinaryTree {
    /*
    問題の理解:
    二分探索木の根が与えられるので、最大深さを返す。
    最大深さとは根から最も遠い葉(子を持たないノード)までの経路上にあるノードの数。

    memo:
    再帰によるDFSで葉を見つけるまでのノードの数をカウントする。
    葉ノードを見つけたら、引数で受け取っておいた可変参照の最大カウント数と比較して更新する。

    Accepted
    */
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        Self::explore_max_depth(root, 0, &mut max_depth);

        max_depth
    }

    fn explore_max_depth(node: Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32) {
        let Some(node) = node else {
            *max_depth = depth.max(*max_depth);
            return;
        };
        let node = node.borrow();
        Self::explore_max_depth(node.left.clone(), depth + 1, max_depth);
        Self::explore_max_depth(node.right.clone(), depth + 1, max_depth);
    }
}
