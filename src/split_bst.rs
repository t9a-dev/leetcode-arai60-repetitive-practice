use std::{cell::RefCell, cmp::Ordering, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(PartialEq, Eq, Debug)]
struct TreeNode {
    val: u32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

impl TreeNode {
    pub fn build_node(val: u32) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    pub fn count_nodes(&self) -> usize {
        let mut count = 1;
        let mut frontier = Vec::from_iter([
            self.left.as_ref().map(Rc::clone),
            self.right.as_ref().map(Rc::clone),
        ]);
        while let Some(node) = frontier.pop() {
            let Some(node) = node else {
                continue;
            };
            count += 1;

            let node = node.borrow();
            frontier.push(node.left.as_ref().map(Rc::clone));
            frontier.push(node.right.as_ref().map(Rc::clone));
        }

        count
    }

    fn new(val: u32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count_nodes()
            .cmp(&other.count_nodes())
            .then(self.val.cmp(&other.val))
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct SplitBst {}
impl SplitBst {
    /*
    https://www.lintcode.com/problem/847/description
    問題の理解:
    二分探索木(BST)の根root、整数vが与えられる。
    BSTをv以下の部分木、vを超える値からなる部分木に分割する。
    分割した部分木のうち、ノード数が多い方を答えとして返す。ノード数が等しい場合はrootノードの値が大きい方を返す。
    BSTを分割する前後でノードの親子関係が変わってはならない。

    memo:
    BSTの分割と分割後の部分木のノード数をカウントする処理に分けて考えないと複雑になって解けなさそう。
    v以下であれば左部分木、それ以外は右部分木に分けていく。
    実装が思いつかず手が止まったので答えを見る。

    解法の理解:
    見ているノードの値がv以下であれば、左サブツリーとして扱う。
    BSTなので、右ノードはより大きいノードの集合（部分木）であることが確定している。
    右ノードを起点に再帰で処理して戻り値の左部分木を今見ているノードの右部分木として扱う。
    見ているノードの値がvより大きければ逆を行う。

    */
    pub fn split_bst(root: TreeNodeRef, v: u32) -> TreeNodeRef {
        let (left_tree, right_tree) = Self::split_tree(Some(root), v);
        let (left_tree, right_tree) = match (left_tree, right_tree) {
            (Some(l), Some(r)) => (l, r),
            (Some(l), None) => return l,
            (None, Some(r)) => return r,
            (None, None) => unreachable!(),
        };

        let left_node_counts = left_tree.borrow().count_nodes();
        let right_node_counts = right_tree.borrow().count_nodes();
        if left_node_counts == right_node_counts {
            if left_tree.borrow().val < right_tree.borrow().val {
                return right_tree;
            }
            return left_tree;
        }

        if left_node_counts < right_node_counts {
            return right_tree;
        }
        left_tree
    }

    fn split_tree(node: Option<TreeNodeRef>, v: u32) -> (Option<TreeNodeRef>, Option<TreeNodeRef>) {
        let Some(node) = node else {
            return (None, None);
        };
        let node_value = node.borrow().val;

        if node_value <= v {
            let right_node = node.borrow_mut().right.take();
            let (left_subtree, right_subtree) = Self::split_tree(right_node, v);

            node.borrow_mut().right = left_subtree;
            (Some(node), right_subtree)
        } else {
            let left_node = node.borrow_mut().left.take();
            let (left_subtree, right_subtree) = Self::split_tree(left_node, v);

            node.borrow_mut().left = right_subtree;
            (left_subtree, Some(node))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    fn to_binary_search_tree(node_values: &Vec<Option<u32>>) -> TreeNodeRef {
        let get_node_value = |i: usize| -> Option<u32> {
            let Some(node_value) = node_values.get(i) else {
                return None;
            };
            *node_value
        };
        let Some(root_node_value) = get_node_value(0) else {
            panic!("require root node value");
        };
        let root = TreeNode::build_node(root_node_value);
        let mut nodes = VecDeque::from_iter([Rc::clone(&root)]);

        let mut i = 1;
        while let Some(node) = nodes.pop_front() {
            let mut node = node.borrow_mut();

            if let Some(left_node_value) = get_node_value(i) {
                let left_node = TreeNode::build_node(left_node_value);
                node.left = Some(Rc::clone(&left_node));
                nodes.push_back(left_node);
            }
            i += 1;

            if let Some(right_node_value) = get_node_value(i) {
                let right_node = TreeNode::build_node(right_node_value);
                node.right = Some(Rc::clone(&right_node));
                nodes.push_back(right_node);
            }
            i += 1;
        }

        root
    }

    fn to_node_values(root: Rc<RefCell<TreeNode>>) -> Vec<Option<u32>> {
        let mut nodes = VecDeque::from_iter([Some(root)]);
        let mut node_values = vec![];

        while let Some(node) = nodes.pop_front() {
            let Some(node) = node else {
                node_values.push(None);
                continue;
            };

            let node = node.borrow();
            node_values.push(Some(node.val));
            nodes.push_back(node.left.as_ref().map(Rc::clone));
            nodes.push_back(node.right.as_ref().map(Rc::clone));
        }

        while node_values.last().is_some_and(|v| v.is_none()) {
            node_values.pop();
        }

        node_values
    }

    #[test]
    fn split_bst_test_1() {
        // BST: [4,2,6,1,3,5,7]
        // v=2: <=2 is {2,1}, >2 is {4,3,6,5,7}
        let root = to_binary_search_tree(&vec![
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
        ]);
        let picked = SplitBst::split_bst(root, 2);
        assert_eq!(picked.borrow().val, 4);
        assert_eq!(picked.borrow().count_nodes(), 5);

        // v=4: <=4 is {4,2,1,3}, >4 is {6,5,7}
        let root = to_binary_search_tree(&vec![
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
        ]);
        let picked = SplitBst::split_bst(root, 4);
        assert_eq!(picked.borrow().val, 4);
        assert_eq!(picked.borrow().count_nodes(), 4);
    }

    #[test]
    fn split_bst_test_2() {
        // BST: [5,2,7,1,3,6]
        // v=4: <=4 is {2,1,3}, >4 is {5,7,6}
        let root =
            to_binary_search_tree(&vec![Some(5), Some(2), Some(7), Some(1), Some(3), Some(6)]);
        let picked = SplitBst::split_bst(root, 4);
        assert_eq!(picked.borrow().val, 5);
        assert_eq!(picked.borrow().count_nodes(), 3);

        // BST: [5,2,7,1,3,6]
        // v=1: <=1 is {1}, >1 is {5,2,7,#,3,6}
        let root =
            to_binary_search_tree(&vec![Some(5), Some(2), Some(7), Some(1), Some(3), Some(6)]);
        let picked = SplitBst::split_bst(root, 1);
        assert_eq!(picked.borrow().val, 5);
        assert_eq!(picked.borrow().count_nodes(), 5);
        assert_eq!(
            to_node_values(picked),
            vec![Some(5), Some(2), Some(7), None, Some(3), Some(6)]
        );
    }

    #[test]
    fn to_nodes_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(to_node_values(root), node_values);
    }

    #[test]
    fn to_binary_search_tree_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let actual_root = to_binary_search_tree(&node_values);

        let expect_root = TreeNode::build_node(5);
        expect_root.borrow_mut().right = Some(TreeNode::build_node(7));
        expect_root
            .borrow_mut()
            .right
            .as_ref()
            .map(Rc::clone)
            .unwrap()
            .borrow_mut()
            .left = Some(TreeNode::build_node(6));

        assert_eq!(actual_root, expect_root);
    }

    #[test]
    fn count_nodes_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(root.borrow().count_nodes(), 3);

        let node_values = vec![Some(5), None, Some(7)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(root.borrow().count_nodes(), 2);

        let node_values = vec![Some(5)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(root.borrow().count_nodes(), 1);
    }
}
