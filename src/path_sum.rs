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
struct PathSum {}
impl PathSum {
    /*
    問題の理解:
    二分木の根と整数target_sumが与えられる。
    根から葉（子を持たないノード）までの経路上のノードの値の合計とtarget_sumが等しくなる経路があればtrueを返す。
    なければfalseを返す。

    memo:
    再帰によるDFSで経路上のノードの値を加算しながら葉ノードのときに合計値とtarget_sumを比較してboolを返す。
    結果をorでまとめる
    target_sumがi32なので、target_sumからノードの値を減算していって、葉ノードの時に0かどうかを見ても良さそう。

    Accept
    */
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(node) = root else {
            return false;
        };
        let node = node.borrow();
        let is_leaf = node.left.is_none() && node.right.is_none();
        if is_leaf {
            return target_sum - node.val == 0;
        }

        Self::has_path_sum(node.left.as_ref().map(Rc::clone), target_sum - node.val)
            || Self::has_path_sum(node.right.as_ref().map(Rc::clone), target_sum - node.val)
    }
}
