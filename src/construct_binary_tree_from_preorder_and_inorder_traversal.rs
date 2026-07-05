use std::{cell::RefCell, rc::Rc};

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

struct ConstructAndBinaryTreeFromPreorderAndInorderTraversal {}
impl ConstructAndBinaryTreeFromPreorderAndInorderTraversal {
    /*
    問題の理解:
    2つの整数配列`preorder`,`inorder`が与えられる。
    `preorder`は二分木の前順走査、`inorder`は同じ木の中順走査を表している。
    これらの情報を元に二分木を構築して返す。

    memo:
    preorder[0]がrootの値になるのは分かる。
    方針が全く思いつかず手が止まったので答えを見る。

    解法の理解:
    - preorder[0]で根が決まる。
        - ここが不変条件になっているので、常にpreorder[0]がノードの値となるようにする。
    - 根が決まるとinorder[i] == preorder[0]のとき、左部分木inorder[..preorder[0]],右部分木inorder[preorder[0]+1..]が分かる

    preorder[0]がノードの値になるという不変条件が分かるか、左部分木と右部分木に分かるという発想ができるかという感じ。
    preorder, inorderと二分木の図を渡されても、この解き方を覚えてないと自力で解法にたどり着ける気がしない。
    */
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_binary_tree(&preorder, &inorder)
    }

    fn build_binary_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let node_value = preorder.first()?;
        let inorder_node_value_index = inorder.iter().position(|v| v == node_value)?;

        let inorder_left = &inorder[..inorder_node_value_index];
        let inorder_right = &inorder[inorder_node_value_index + 1..];

        let preorder_left = &preorder[1..inorder_left.len() + 1];
        let preorder_right = &preorder[inorder_left.len() + 1..];

        let mut node = TreeNode::new(*node_value);
        node.left = Self::build_binary_tree(preorder_left, inorder_left);
        node.right = Self::build_binary_tree(preorder_right, inorder_right);

        Some(Rc::new(RefCell::new(node)))
    }
}
