use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
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

struct MergeTwoBinaryTrees {}
impl MergeTwoBinaryTrees {
    /*
     * 問題の理解:
     * 二分木の根root1,root2が与えられる。
     * 2つの二分木をマージして新しい木を作って返す。
     * マージのルールとして重なり合う位置にあるノードの値は、合計して新しいノードとして扱う。
     * 片方にしか無いノードの場合は片方のノードを使う。
     *
     * memo:
     * BFSで同じ深さ毎にノードを見ていって新しいノードを組み立てて行けば良さそう。
     *
     * WrongAnswerとなった。マージ後のrootノードに対して正しくノードを挿入できておらず戻り値が空(None)になる。
     * 答えを見る。
     *
     * 所感:
     * 木全体をマージすると考えるより、個々のノードの子ノードを操作すると言う感じで分解して考えられると良さそう。
     */
    pub fn merge_trees(
        root1: Option<TreeNodeRef>,
        root2: Option<TreeNodeRef>,
    ) -> Option<TreeNodeRef> {
        let Some(root1) = root1 else {
            return root2;
        };
        let Some(root2) = root2 else {
            return Some(root1);
        };
        let create_tree_node_ref =
            |val: i32| -> TreeNodeRef { Rc::new(RefCell::new(TreeNode::new(val))) };
        let merged_root = create_tree_node_ref(root1.borrow().val + root2.borrow().val);
        let mut frontier = VecDeque::from_iter([(
            Rc::clone(&root1),
            Rc::clone(&root2),
            Rc::clone(&merged_root),
        )]);

        while let Some((node1, node2, merged_node)) = frontier.pop_front() {
            let node1 = node1.borrow();
            let node2 = node2.borrow();
            match (&node1.left, &node2.left) {
                (None, None) => (),
                (None, Some(left2)) => {
                    merged_node.borrow_mut().left = Some(Rc::clone(left2));
                }
                (Some(left1), None) => {
                    merged_node.borrow_mut().left = Some(Rc::clone(left1));
                }
                (Some(left1), Some(left2)) => {
                    let merged_left = create_tree_node_ref(left1.borrow().val + left2.borrow().val);
                    merged_node.borrow_mut().left = Some(Rc::clone(&merged_left));
                    frontier.push_back((
                        Rc::clone(left1),
                        Rc::clone(left2),
                        Rc::clone(&merged_left),
                    ));
                }
            }

            match (&node1.right, &node2.right) {
                (None, None) => (),
                (None, Some(right2)) => {
                    merged_node.borrow_mut().right = Some(Rc::clone(right2));
                }
                (Some(right1), None) => {
                    merged_node.borrow_mut().right = Some(Rc::clone(right1));
                }
                (Some(right1), Some(right2)) => {
                    let merged_right =
                        create_tree_node_ref(right1.borrow().val + right2.borrow().val);
                    merged_node.borrow_mut().right = Some(Rc::clone(&merged_right));
                    frontier.push_back((
                        Rc::clone(right1),
                        Rc::clone(right2),
                        Rc::clone(&merged_right),
                    ));
                }
            }
        }

        Some(merged_root)
    }

    pub fn merge_trees_recursive(
        root1: Option<TreeNodeRef>,
        root2: Option<TreeNodeRef>,
    ) -> Option<TreeNodeRef> {
        let Some(node1) = root1 else {
            return root2;
        };
        let Some(node2) = root2 else {
            return Some(node1);
        };
        let node1 = node1.borrow();
        let node2 = node2.borrow();
        let merged_node = Rc::new(RefCell::new(TreeNode::new(node1.val + node2.val)));
        merged_node.borrow_mut().left = Self::merge_trees_recursive(
            node1.left.as_ref().map(Rc::clone),
            node2.left.as_ref().map(Rc::clone),
        );
        merged_node.borrow_mut().right = Self::merge_trees_recursive(
            node1.right.as_ref().map(Rc::clone),
            node2.right.as_ref().map(Rc::clone),
        );

        Some(merged_node)
    }
}
