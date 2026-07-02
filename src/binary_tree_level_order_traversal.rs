use std::{cell::RefCell, collections::VecDeque, rc::Rc};

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct BinaryTreeLevelOrderTraversal {}
impl BinaryTreeLevelOrderTraversal {
    /*
    問題の理解:
    二分木の根が与えられたとき、そのノード値をレベル順（左から右、レベルごとに）並べたリストを返す。

    memo:
    BFSでレベルごとに見ていくのが素直だと思う。

    Wrong Answerとなった。解の考え方自体が間違っているように見える。
    解のインデックス=レベルとして、あるレベルのノードの値はすべて同じインデックスの値とする必要がありそう。

    Accepted
    問題で求められている解を正しく理解できていなかったのと、時間がかかったのでNGとしておく
    */
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };
        let mut level_order_node_values: Vec<Vec<_>> = vec![];
        let mut frontier = VecDeque::from_iter([(root, 0)]);
        while let Some((node, level)) = frontier.pop_front() {
            let node = node.borrow();
            match level_order_node_values.get_mut(level) {
                Some(node_values) => node_values.push(node.val),
                None => level_order_node_values.push(vec![node.val]),
            }

            if let Some(left) = &node.left {
                frontier.push_back((Rc::clone(left), level + 1));
            }
            if let Some(right) = &node.right {
                frontier.push_back((Rc::clone(right), level + 1));
            }
        }

        level_order_node_values
    }
}
