use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;

struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

struct BinaryTreeLevelTraversal {}

impl BinaryTreeLevelTraversal {
    /*
     * 問題の理解:
     * 二分木の根が与えられるので、ノードの値のレベル順（左から右へ階層毎に）を配列で返す。
     *
     * memo:
     * 答えを格納する配列に対して、ノードを根から走査する。
     * あるノードに注目した時、左子ノード -> 右子ノード の順に配列として答えの配列にpushする。
     * Wrong Answerとなった。
     * in = [1,2,3,4,null,null,5], expect = [[1],[2,3],[4,5]], actual = [[1],[2,3],[4],[5]]
     * 同じ階層のノードの値は全て同じ配列に入っているように見えるので、この考え方でコードを修正する。
     * Accepted
     * 問題の内容の理解から間違えていた。
     */
    pub fn level_order(root: Option<TreeNodeRef>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };
        let mut level_order_node_values: Vec<Vec<_>> = vec![vec![root.borrow().val]];
        let mut frontier = VecDeque::from_iter([(root, 0usize)]);
        while let Some((node, level)) = frontier.pop_front() {
            let node = node.borrow();
            let next_level = level + 1;
            let mut node_values = vec![];

            if let Some(left) = &node.left {
                node_values.push(left.borrow().val);
                frontier.push_back((Rc::clone(left), next_level));
            }
            if let Some(right) = &node.right {
                node_values.push(right.borrow().val);
                frontier.push_back((Rc::clone(right), next_level));
            }
            if node_values.is_empty() {
                continue;
            }
            match level_order_node_values.get_mut(next_level) {
                Some(exist_values) => exist_values.extend(node_values),
                None => level_order_node_values.push(node_values),
            }
        }

        level_order_node_values
    }

    /*
     * next_levelの部分が冗長になるだけで意味がないと感じたのでリファクタリング
     */
    pub fn level_order2(root: Option<TreeNodeRef>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };
        let mut level_order_node_values = vec![vec![]];
        let mut frontier = VecDeque::from_iter([(root, 0usize)]);
        while let Some((node, level)) = frontier.pop_front() {
            let node = node.borrow();
            match level_order_node_values.get_mut(level) {
                Some(level_order_values) => level_order_values.push(node.val),
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
