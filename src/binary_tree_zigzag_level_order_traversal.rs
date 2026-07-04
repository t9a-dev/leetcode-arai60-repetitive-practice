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

struct BinaryTreeZigzagLevelOrderTraversal {}
impl BinaryTreeZigzagLevelOrderTraversal {
    /*
    問題の理解:
    二分木の根が与えられる。
    根からスタートして左->右、次のレベルでは逆の右->左、次のレベルでは左->右...といったようにジグザグに探索する。
    レベルごとのノードの配列を生成して返す。

    memo:
    BFSでレベルごとにdequeにpushする順番を制御すればよさそう。
    Directionというenumを定義して方向の定義を行う。

    Wrong Answerとなった。Directionの切替方法が間違っている。レベル毎に切り替えるべきだが、毎回今と違う方向に切り替える実装になっている。
    Dequeにdirectionを引き継ぐ必要はなくて、レベルが偶数か奇数かで切り替えることでレベルごとに切り替えられる。

    Wrong Answerとなった。答えを見る。
    以下のような解法のバリエーションがあった。
    - レベルごとにノードの値を持つデータ構造をstackではなく、dequeでもってpush_front,push_backにより左優先、右優先を表現する。
    - すべてのレベルを左優先探索して、奇数レベルのノードの配列をreverseする。

    */
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };
        let mut zigzag_level_order_node_values: Vec<VecDeque<_>> = Vec::new();
        let mut frontier = VecDeque::from_iter([(root, 0)]);
        while let Some((node, level)) = frontier.pop_front() {
            let node = node.borrow();
            match zigzag_level_order_node_values.get_mut(level) {
                Some(node_values) => {
                    if level % 2 == 0 {
                        node_values.push_back(node.val);
                    } else {
                        node_values.push_front(node.val);
                    }
                }
                None => zigzag_level_order_node_values.push(VecDeque::from_iter([node.val])),
            }

            if let Some(left) = &node.left {
                frontier.push_back((Rc::clone(left), level + 1));
            }
            if let Some(right) = &node.right {
                frontier.push_back((Rc::clone(right), level + 1));
            }
        }

        zigzag_level_order_node_values
            .into_iter()
            .map(|node_values| node_values.into_iter().collect::<Vec<_>>())
            .collect()
    }

    pub fn zigzag_level_order2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };
        let mut level_order_node_values: Vec<Vec<_>> = Vec::new();
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

        Self::to_zigzag_level_order(&mut level_order_node_values);
        level_order_node_values
    }

    fn to_zigzag_level_order(level_order_node_values: &mut [Vec<i32>]) {
        for (level, node_values) in level_order_node_values.iter_mut().enumerate() {
            if level % 2 == 0 {
                continue;
            }
            node_values.reverse();
        }
    }
}
