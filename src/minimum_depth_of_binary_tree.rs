use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}
struct MinimumDepthOfBinaryTree {}
impl MinimumDepthOfBinaryTree {
    /*
     * 問題の理解:
     * 二分木が与えられるので根から一番近い葉ノードまでの深さを返す。
     * 葉ノードとは子ノードを持たないノードのこと。
     * 根ノードが深さ1となる。
     *
     * memo:
     * 最小深さを見ればよいので、BFSが適していると思う。
     * ノードを深さ毎に見ていってleaf nodeであればその時点の深さを返す。
     *
     * [3,9,20,null,null,15,7] expected = 2, actual = 3 でWrong Answer
     * pop_frontで取り出すべきところをpop_backしていたのを修正してAccepted
     * 間違えたのでNG
     */
    pub fn min_depth(root: Option<TreeNodeRef>) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        let mut frontier = VecDeque::from_iter([(root, 1)]);
        while let Some((node, depth)) = frontier.pop_front() {
            let node = node.borrow();
            let is_leaf = node.left.is_none() && node.right.is_none();
            if is_leaf {
                return depth;
            }

            if let Some(left) = &node.left {
                frontier.push_back((Rc::clone(left), depth + 1));
            }
            if let Some(right) = &node.right {
                frontier.push_back((Rc::clone(right), depth + 1));
            }
        }

        0
    }
}
