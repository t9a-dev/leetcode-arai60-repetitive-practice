use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}
struct RemoveDuplicatesFromSortedList {}
impl RemoveDuplicatesFromSortedList {
    /*
    問題の理解:
    ソート済連結リストの先頭ノードが渡される。
    重複する値を持つノードを削除してソート済連結リストの先頭を返す。

    memo:
    ナイーブな実装だと
    - ノードの値を全て取り出す
    - 重複排除
    - ソート
    - 連結リストを構築
    - 先頭ノードを返す

    WrongAnswerとなった。
    - foldで連結リストを組み立てるときは、末尾からノードの値を見る必要があるので
    ノードの値を逆順で走査する必要があることを見落としていた
    - HashSetがinsertの順序を保持しないことを見落としていた

    値を集めるのではなく、ポインタを辿る解法をも書いておく。
    */
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let head = head?;
        let mut node_values = HashSet::new();
        node_values.insert(head.val);
        let mut tail = head;
        while let Some(node) = tail.next {
            node_values.insert(node.val);
            tail = node;
        }

        let mut node_values = node_values.into_iter().collect::<Vec<_>>();
        node_values.sort();
        node_values.reverse();
        node_values.into_iter().fold(None, |child, node_value| {
            let mut parent = ListNode::new(node_value);
            parent.next = child;
            Some(Box::new(parent))
        })
    }

    /*
    headを可変参照で受け取って、連結リストを操作しながら重複する値を持つノードを飛ばす解法
    */
    pub fn delete_duplicates2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tail = head.as_mut();
        while let Some(node) = tail {
            while let Some(next_node) = node.next.as_mut() {
                if node.val != next_node.val {
                    break;
                }
                node.next = next_node.next.take();
            }
            tail = node.next.as_mut();
        }

        head
    }
}
