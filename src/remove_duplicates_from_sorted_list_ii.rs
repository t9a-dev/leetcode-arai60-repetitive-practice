use std::collections::HashMap;

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

pub struct RemoveDuplicatesFromSortedList {}
impl RemoveDuplicatesFromSortedList {
    /*
    memo:
    単方向連結リストから重複する値を持つノードは全て取り除く
    1 -> 2 -> 2-> 3 は 1 -> 3 になる
    単方向連結リストを先頭から操作しつつ、HashSetに入れる
    返却用の空のdummy_headを作って重複が無いことが分かったら、dummyに繋いでいく。
    dummy.nextを返す
    手が止まったので写経する。
    HashSetでは重複した値をそのまま取り除くことができない。

    コードの理解:
    ノードの値の出現回数をHashMapで管理する。
    出現回数が1の値を抽出してソートし、単方向連結リストを組み立てて返す。
    単方向連結リストは底から根に向かって組み立てている。
    node.nextはOptionなので、Some(T),Noneの条件分岐が発生するのに比べて、子(next)から見た時の根は常に存在するためコードがシンプルになる。

    n = nodes.len
    時間計算量: O(n)
    補助空間計算量: O(n)
    */
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node_value_to_count = HashMap::new();
        let mut current = head;
        while let Some(node) = current {
            node_value_to_count
                .entry(node.val)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            current = node.next;
        }

        let mut unique_node_values = node_value_to_count
            .into_iter()
            .filter_map(|(value, count)| (count == 1).then_some(value))
            .collect::<Vec<_>>();
        unique_node_values.sort();

        unique_node_values
            .into_iter()
            .rev()
            .fold(None, |child, node_value| {
                let mut parent = Box::new(ListNode::new(node_value));
                parent.next = child;
                Some(parent)
            })
    }

    /*
    コードの理解:
    書き込み用のdummy_head（番兵ノード）からスタートする
    今見ているノードの値と次のノードの値が重複している場合は次のノードを捨てるために、次の次のノードを次のノードに設定。
    この時点で今見ているノードは重複するノードを持っているので記録しておく（has_duplicate=true）
    重複がなくなるまで繰り返す。
    今見ていたノード自体が重複していた(has_duplicate==true)ときは、次の次のノードを次のノードに設定することで重複していた値をもつノードを捨てる。

    memo:
    Rustの所有権システムの関係で以下のあたりを意識して書く必要がある。
    - as_mut()による可変参照(&mut)
    - take()による所有権移動
    - insert()の戻り値で可変参照(&mut)を得ている

    n = nodes.len
    時間計算量: O(n)
    補助空間計算量: O(1)
    */
    pub fn delete_duplicates_in_place(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;
        let mut current = dummy_head.as_mut();

        while let Some(mut node) = current.next.take() {
            let mut has_duplicate = false;

            while let Some(next_node) = node.next.as_mut() {
                if node.val != next_node.val {
                    break;
                }

                has_duplicate = true;
                node.next = next_node.next.take();
                continue;
            }

            if has_duplicate {
                current.next = node.next;
            } else {
                current = current.next.insert(node);
            }
        }

        dummy_head.next
    }
}
