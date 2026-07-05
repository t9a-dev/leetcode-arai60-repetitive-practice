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

struct ValidateBinarySearchTree {}
impl ValidateBinarySearchTree {
    /*
    問題の理解:
    二分木の根が与えられる。二分木が二分探索木であるかをbool値で返す。
    二分探索木の条件
        - ノードの左部分木には、そのノードの値よりも厳密に小さい値を持つノードのみが含まれる
        - ノードの右部分木には、そのノードの値よりも厳密に大きい値を持つノードのみが含まれる
        - 左右それぞれ部分木もそれぞれ有効な二分探索木である必要がある。

    memo:
    DFSによる再帰処理を行いながら、あるノードの左右のノードが条件満たしているかを再帰的に見ることで判定できると思う。
    最終的にすべての再帰処理の戻り値をAndで確認して返す。
    再帰のbase caseでノードが空のときはBSTの条件に違反していないのでtrueにすることにした。

    Wrong Answerとなった。
    左部分木に含まれるノードの値が根から見た時にすべて小さい値になっていないことを検証できていない。
    右部分木に含まれるノードの値が根からみた時にすべて大きい値になっていないことを検証できていない。
    1つ上のノードと比較すれば正しく検証できる？
    時間切れなので答えを見る。

    解法の理解:
    ノードとノード自身が持つ左右の子の値を見るのではなく、下限上限という見方をしている。
    左の子を見るときは、ノードの値を上限として更新しながら判定している。
    右の子を見るときは、ノードの値を下限として更新しながら判定している。

    下限、上限の設定がないときは確認しなくて良い、設定があるときは設定値を満たすかを確認するというのをOption型とis_none_orメソッドでいい感じに表現できた。

    */
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return true;
        };
        Self::validate_bst(root, None, None)
    }

    fn validate_bst(
        node: Rc<RefCell<TreeNode>>,
        lower_limit: Option<i32>,
        upper_limit: Option<i32>,
    ) -> bool {
        let node = node.borrow();
        let is_bst = lower_limit.is_none_or(|lower| lower < node.val)
            && upper_limit.is_none_or(|upper| node.val < upper);
        let is_bst_left = match &node.left {
            Some(left) => Self::validate_bst(Rc::clone(left), lower_limit, Some(node.val)),
            None => true,
        };
        let is_bst_right = match &node.right {
            Some(right) => Self::validate_bst(Rc::clone(right), Some(node.val), upper_limit),
            None => true,
        };

        is_bst && is_bst_left && is_bst_right
    }

    pub fn is_valid_bst2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return true;
        };
        Self::validate_bst2(root, None, None)
    }

    // 早期リターンによる枝刈り
    fn validate_bst2(
        node: Rc<RefCell<TreeNode>>,
        lower_limit: Option<i32>,
        upper_limit: Option<i32>,
    ) -> bool {
        let node = node.borrow();
        let is_bst = lower_limit.is_none_or(|lower| lower < node.val)
            && upper_limit.is_none_or(|upper| node.val < upper);
        if !is_bst {
            return false;
        }

        let is_bst_left = match &node.left {
            Some(left) => Self::validate_bst2(Rc::clone(left), lower_limit, Some(node.val)),
            None => true,
        };
        if !is_bst_left {
            return false;
        }

        match &node.right {
            Some(right) => Self::validate_bst2(Rc::clone(right), Some(node.val), upper_limit),
            None => true,
        }
    }
}
