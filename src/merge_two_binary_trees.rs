use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    rc::Rc,
};

struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

struct MergeTwoBinaryTreesDfs {}
impl MergeTwoBinaryTreesDfs {
    /*
    問題の理解:
    2つの二分木の根root1,root2が与えられる。2つの二分木を統合したときの二分木の根を返す。
    統合ルールは以下。
    - 2つのノードが重なる時は、ノードの値を合計する
    - それ以外の時、Noneではないノードを新しい木のノードとして利用する

    memo:
    引数のシグネチャから2つの引数に変更を加えると、呼び出し元に影響するのでどちらかの木をベースにしてマージするのは止めた方が良いと判断。
    BFSによる処理で、キューに[(root1,root2,merged_root)]という感じで層ごとに入れていって都度マージする方法が分かりやすそう。

    Accepted
    ただ、BFSでやろうとして1時間ほど迷った挙句、書ききれずにDFSで書きなおしてAcceptedという感じなのでNGにしておく。
    BFSで書くべきところを〜というよりは、この方向で解けそうだという感覚と、実装できることに乖離があり練習不足を感じる。
    */
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut merge_root = None;
        Self::merge_tree(root1, root2, &mut merge_root);
        merge_root
    }

    fn merge_tree(
        node1: Option<Rc<RefCell<TreeNode>>>,
        node2: Option<Rc<RefCell<TreeNode>>>,
        merge_node: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        match (node1, node2) {
            (Some(n1), Some(n2)) => {
                let (n1, n2) = (n1.borrow(), n2.borrow());
                let merged_node = merge_node.insert(Self::create_node(n1.val + n2.val));
                Self::merge_tree(
                    n1.left.clone(),
                    n2.left.clone(),
                    &mut merged_node.borrow_mut().left,
                );
                Self::merge_tree(
                    n1.right.clone(),
                    n2.right.clone(),
                    &mut merged_node.borrow_mut().right,
                );
            }
            (Some(n1), None) => {
                let n1 = n1.borrow();
                let merged_node = merge_node.insert(Self::create_node(n1.val));
                Self::merge_tree(n1.left.clone(), None, &mut merged_node.borrow_mut().left);
                Self::merge_tree(n1.right.clone(), None, &mut merged_node.borrow_mut().right);
            }
            (None, Some(n2)) => {
                let n2 = n2.borrow();
                let merged_node = merge_node.insert(Self::create_node(n2.val));
                Self::merge_tree(None, n2.left.clone(), &mut merged_node.borrow_mut().left);
                Self::merge_tree(None, n2.right.clone(), &mut merged_node.borrow_mut().right);
            }
            _ => (),
        }
    }

    fn create_node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    // 過去に書いた再帰のコードを写経
    pub fn merge_trees2(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root1) = root1 else {
            return root2;
        };
        let Some(root2) = root2 else {
            return Some(root1);
        };
        let merged_root = Self::create_node(root1.borrow().val + root2.borrow().val);
        merged_root.borrow_mut().left = Self::merge_trees2(
            root1.borrow().left.as_ref().map(Rc::clone),
            root2.borrow().left.as_ref().map(Rc::clone),
        );
        merged_root.borrow_mut().right = Self::merge_trees2(
            root1.borrow().right.as_ref().map(Rc::clone),
            root2.borrow().right.as_ref().map(Rc::clone),
        );

        Some(merged_root)
    }
}

struct MergeTwoBinaryTreesBfs {}
impl MergeTwoBinaryTreesBfs {
    // 写経
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root1) = root1 else {
            return root2;
        };
        let Some(root2) = root2 else {
            return Some(root1);
        };
        let merged_root = Self::create_node(root1.borrow().val + root2.borrow().val);
        let mut frontier = VecDeque::from_iter([(
            Rc::clone(&root1),
            Rc::clone(&root2),
            Rc::clone(&merged_root),
        )]);

        while let Some((node1, node2, merged_node)) = frontier.pop_front() {
            let (node1, node2) = (node1.borrow(), node2.borrow());
            match (&node1.left, &node2.left) {
                (Some(left1), Some(left2)) => {
                    let merged_left = Self::create_node(left1.borrow().val + left2.borrow().val);
                    merged_node.borrow_mut().left = Some(Rc::clone(&merged_left));
                    frontier.push_back((
                        Rc::clone(left1),
                        Rc::clone(left2),
                        Rc::clone(&merged_left),
                    ));
                }
                (Some(left1), None) => merged_node.borrow_mut().left = Some(Rc::clone(left1)),
                (None, Some(left2)) => merged_node.borrow_mut().left = Some(Rc::clone(left2)),
                _ => (),
            }

            match (&node1.right, &node2.right) {
                (Some(right1), Some(right2)) => {
                    let merged_right = Self::create_node(right1.borrow().val + right2.borrow().val);
                    merged_node.borrow_mut().right = Some(Rc::clone(&merged_right));
                    frontier.push_back((
                        Rc::clone(right1),
                        Rc::clone(right2),
                        Rc::clone(&merged_right),
                    ));
                }
                (Some(right1), None) => merged_node.borrow_mut().right = Some(Rc::clone(right1)),
                (None, Some(right2)) => merged_node.borrow_mut().right = Some(Rc::clone(right2)),
                _ => (),
            }
        }

        Some(merged_root)
    }

    fn create_node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }
}
