use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct MinimumDepthOfBinaryTree {}
impl MinimumDepthOfBinaryTree {
    /*
    問題の理解:
    二分木が与えられたとき、その最小深度を求める。
    最小深度とは根から最も近い葉（子を持たないノード）までの経路上にあるノードの数のこと。

    memo:
    根から辿っていって一番最初に見つけた葉ノードまでの経路上になるノード数を知りたいので、BFSで葉ノードを見つけたら即リターンする。

    葉を見つけた時に探索打ち切りにするべきところを、Noneを見つけた時に探索打ち切りしていしまい Wrong Answerとなった。
    修正しAcceptedとなったが、NGとして記録しておく

    */
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        let mut frontier = VecDeque::from_iter([(root, 1)]);
        while let Some((node, depth)) = frontier.pop_front() {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return depth;
            }
            if let Some(left_node) = node.left.clone() {
                frontier.push_back((left_node.clone(), depth + 1));
            }
            if let Some(right_node) = node.right.clone() {
                frontier.push_back((right_node.clone(), depth + 1));
            }
        }

        unreachable!()
    }
}
