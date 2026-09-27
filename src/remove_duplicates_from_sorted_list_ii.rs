use core::net;
use std::collections::HashMap;

struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}
struct RemoveDuplicatesFromSortedListII {}
impl RemoveDuplicatesFromSortedListII {
    /*
    問題の理解:
    ソート済の連結リストの先頭ノードを与えられる。
    重複する値を持つノードを全て削除して、ソート済の連結リストの先頭ノードを返す。
    1 -> 2 -> 2 -> 3 のとき、2は重複しているので全て削除して 1 -> 3 を返す。

    memo:
    先頭ノードが変わり得る。
    dummy_headを作って、dummy.nextを答えとして返せば良さそう。
    ポインタを使って辿っていく解法は思いつかないのでナイーブな実装を行う。
    HashMapでノードの値と出現回数を作る。
    出現回数が1の値を集める。
    ソートする。
    答えの連結リストを作って返す。
    Accepted

    ポインタを辿っていく解法を考えてみる。
    手が止まったので写経する。
    */
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node_value_to_count: HashMap<i32, usize> = HashMap::new();
        let mut tail = head;
        while let Some(node) = tail {
            node_value_to_count
                .entry(node.val)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            tail = node.next;
        }

        let mut unique_node_values = node_value_to_count
            .into_iter()
            .filter_map(|(value, count)| {
                if count == 1 {
                    return Some(value);
                }
                None
            })
            .collect::<Vec<_>>();
        unique_node_values.sort();
        unique_node_values.reverse();
        unique_node_values
            .into_iter()
            .fold(None, |child, node_value| {
                let mut parent = ListNode::new(node_value);
                parent.next = child;
                Some(Box::new(parent))
            })
    }

    /*
    ポインタを辿りながら重複検知をフラグで表す解法。
    重複をみつけたら1つ次のノードを飛ばす。
    重複が無くなり、今まで重複していなければ答えの連結リストに加える。
    重複していたことがあれば、削除する必要があるので飛ばす。

    Rustの所有権周りの難しさが前面に出てきているように感じる。
    take()する位置とタイミング、insertによる値の更新と可変参照の取得部分など。
    */
    pub fn delete_duplicates2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode { val: 0, next: head });
        let mut tail = dummy_head.as_mut();
        while let Some(mut node) = tail.next.take() {
            let mut has_duplicate = false;
            while let Some(next_node) = node.next.as_mut() {
                if node.val != next_node.val {
                    break;
                }
                has_duplicate = true;
                node.next = next_node.next.take();
            }

            if has_duplicate {
                tail.next = node.next.take();
                continue;
            }
            tail = tail.next.insert(node);
            // 上記の一文はinsertメソッドが以下の内容をいい感じに実装している。
            // tail.next = Some(node); // この時点でtail.nextはSomeであることが自明なのでunwrap()してもパニックしない。
            // tail = tail.next.as_mut().unwrap();
        }

        dummy_head.next
    }
}
