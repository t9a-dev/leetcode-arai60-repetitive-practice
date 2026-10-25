use std::{cell::RefCell, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;
struct TreeNode {
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
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

struct ConvertSortedArrayToBinarySearchTree {}
impl ConvertSortedArrayToBinarySearchTree {
    /*
     * 問題の理解:
     * 昇順ソートされた整数からなる配列numsが与えられる。
     * numsから平衡二分探索木を作って根を返す。
     *
     * memo:
     * numsは昇順ソート済なので、真ん中位置の値を根として扱える。
     * あるノードに注目している時、そのノードの値の左にあるか、右にあるかがそのまま左部分木、右部分木になっている。
     * 配列の中央に注目する
     * 再帰的にnumsが空になるまで分割してく。
     * nums.len == 1 のときはそのままノードを返す。
     * この手順で再帰処理を書けば解けると思う。
     * Accepted
     */
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<TreeNodeRef> {
        Self::build_bst(&nums)
    }

    fn create_tree_node_ref(val: i32) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    fn build_bst(nums: &[i32]) -> Option<TreeNodeRef> {
        if nums.is_empty() {
            return None;
        };
        if nums.len() == 1
            && let Some(val) = nums.first()
        {
            return Some(Self::create_tree_node_ref(*val));
        };

        let middle = nums.len() / 2;
        let node = Self::create_tree_node_ref(nums[middle]);
        {
            let mut node = node.borrow_mut();
            node.left = Self::build_bst(&nums[0..middle]);
            node.right = Self::build_bst(&nums[middle + 1..nums.len()]);
        }

        Some(node)
    }
}
