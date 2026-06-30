use std::{cell::RefCell, rc::Rc};

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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

struct ConvertSortedArrayToBinarySearchTreeDfs {}
impl ConvertSortedArrayToBinarySearchTreeDfs {
    /*
    問題の理解:
    昇順にソートされた整数配列numsが与えられるので、平衡二分探索木にして根を返す。

    memo:
    全然わからない。手が止まったので写経する。

    解法の理解:
    配列の真ん中を親ノードとして見て、このノードから見て左側にある数値を左部分木、右側にあるノードを右部分木としている。
    再帰処理でノードの左部分木、右部分木を確定している。
    */
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_binary_search_tree(&nums)
    }

    fn build_binary_search_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let node_value_index = nums.len() / 2;
        let node_value = nums.get(node_value_index)?;
        let (left_tree_values, right_tree_values) =
            (&nums[0..node_value_index], &nums[node_value_index + 1..]);
        let left = Self::build_binary_search_tree(left_tree_values);
        let right = Self::build_binary_search_tree(right_tree_values);
        let node = Rc::new(RefCell::new(TreeNode::new(*node_value)));
        {
            let mut node = node.borrow_mut();
            node.left = left;
            node.right = right;
        }

        Some(node)
    }
}

struct ConvertSortedArrayToBinarySearchTreeBfs {}
impl ConvertSortedArrayToBinarySearchTreeBfs {
    /*
    BFSの練習
    */
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_node_value = Self::get_middle_value(&nums)?;
        let root = Self::create_node(root_node_value);
        let left_and_right = Self::split_left_and_right(&nums);
        let mut frontier = vec![(Rc::clone(&root), left_and_right)];
        while let Some((node, (left, right))) = frontier.pop() {
            if let Some(left_node_value) = Self::get_middle_value(left) {
                let left_node = Self::create_node(left_node_value);
                node.borrow_mut().left = Some(Rc::clone(&left_node));
                let left_and_right = Self::split_left_and_right(left);
                frontier.push((left_node, left_and_right));
            }
            if let Some(right_node_value) = Self::get_middle_value(right) {
                let right_node = Self::create_node(right_node_value);
                node.borrow_mut().right = Some(Rc::clone(&right_node));
                let left_and_right = Self::split_left_and_right(right);
                frontier.push((right_node, left_and_right));
            }
        }

        Some(root)
    }

    fn create_node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    fn get_middle_value(nums: &[i32]) -> Option<i32> {
        nums.get(nums.len() / 2).copied()
    }

    fn split_left_and_right(nums: &[i32]) -> (&[i32], &[i32]) {
        let middle = nums.len() / 2;
        (&nums[0..middle], &nums[middle + 1..])
    }
}
