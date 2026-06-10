#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

/*
    memo:
    重複する値を持つノード取り除きたい。
    head,tailで管理する。
    手が止まったので写経する。

    コードの理解:
    先頭から順にノードを見ていく。
    ノードの持つ値で昇順ソートされているので、今見ているノードと次のノードの値が同じであれば、
    次のノードを指している参照を、次の次のノードへの参照に入れ替える。(次のノードの参照を飛ばす。)
*/

pub struct RemoveDuplicatesFromSortedList {}
impl RemoveDuplicatesFromSortedList {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();

        while let Some(node) = current {
            while let Some(next_node) = node.next.as_mut() {
                if node.val != next_node.val {
                    break;
                }
                node.next = next_node.next.take();
            }
            current = node.next.as_mut();
        }

        head
    }
}
