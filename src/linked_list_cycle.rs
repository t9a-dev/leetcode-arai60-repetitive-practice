use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub struct ListNode {
    next: Option<Rc<RefCell<ListNode>>>,
}
type ListNodeRef = Rc<RefCell<ListNode>>;

struct LinkedListCycle {}
impl LinkedListCycle {
    /*
    問題の理解:
    連結リストの先頭要素headが与えられる。
    連結リストにサイクルが存在するかどうかを判定する。
    サイクルが存在するときはtrueを返す。そうでないときはfalseを返す。

    memo:
    ナイーブな実装として一度訪問したノードをHashSetに入れていき重複したら早期リターンでtrueを返す。
    連結リストを辿っていってNoneに当たるまで走査し終わったらfalseを返す。
    Accepted: LeetCode採点システムが対応していないので、テストコードとLLMに実装を突っ込んで確認

    Follow upで空間計算量O(1)の実装が聞かれているのでフロイドの循環検出法も書いておく。
    */
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let Some(head) = head else {
            return false;
        };
        let mut visited_nodes = HashSet::new();
        visited_nodes.insert(Rc::as_ptr(&head));

        let mut next_node = head.borrow().next.as_ref().map(Rc::clone);
        while let Some(node) = next_node {
            if !visited_nodes.insert(Rc::as_ptr(&node)) {
                return true;
            }
            next_node = node.borrow().next.as_ref().map(Rc::clone);
        }
        false
    }

    /*
    フロイドの循環検出法による空間計算量O(1)となる解法。
    解法の仕組みとしてはslow,fast(slowに対して1つ先行する)ポインタで連結リストを辿っていき、slow==fastとなれば循環ありという解法だと思う。
    大まかな方向しか覚えていないので、紙に書いてデバッグしてみて正しそうなら実装する。
    slow = 0, fast = 1のindexでスタートする。slow += 1; fast += 2; で進めていく。
    slow,fastが指すノードのアドレスが等しくなればtrueとして返す。
    slow,fastがNoneになればループ終了
    Accepted
    LLMのレビューによると、一般的な実装は
    slow = head
    fast = head
    while fast.next != null && fast.next != null {
        slow = fast.next;
        fast = fast.next.next
        if slow == fast {
            return true;
        }
    }
    という感じらしい。
    こちらの方がスッキリしているので実装しておく。
    自分の実装だと、while let Someで1つ進めて、さらにループ内でfastノードを1つ進めることで2つ進むようにしているのが分かりづらいなと思った。

    */
    pub fn has_cycle_floyds_detection(head: Option<ListNodeRef>) -> bool {
        let next_node = |node: &ListNodeRef| -> Option<ListNodeRef> {
            node.as_ref().borrow().next.as_ref().map(Rc::clone)
        };
        let Some(mut slow) = head else {
            return false;
        };
        let Some(mut fast) = next_node(&slow) else {
            return false;
        };

        while let Some(fast_node) = next_node(&fast) {
            if Rc::ptr_eq(&slow, &fast_node) {
                return true;
            };

            let Some(next_slow_node) = next_node(&slow) else {
                return false;
            };
            slow = next_slow_node;

            let Some(next_fast_node) = next_node(&fast_node) else {
                return false;
            };
            fast = next_fast_node;
        }

        false
    }

    /*
    こちらのほうが読みやすい。
    */
    pub fn has_cycle_floyds_detection2(head: Option<ListNodeRef>) -> bool {
        let next_node = |node: &ListNodeRef| -> Option<ListNodeRef> {
            node.as_ref().borrow().next.as_ref().map(Rc::clone)
        };
        let Some(mut slow) = head.as_ref().map(Rc::clone) else {
            return false;
        };
        let Some(mut fast) = head else {
            return false;
        };

        loop {
            let Some(next_slow) = next_node(&slow) else {
                return false;
            };
            let Some(next_next_fast) = next_node(&fast).and_then(|next_fast| next_node(&next_fast))
            else {
                return false;
            };

            slow = next_slow;
            fast = next_next_fast;

            if Rc::ptr_eq(&slow, &fast) {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list_with_cycle(
        cycle_position: Option<usize>,
        list_len: usize,
    ) -> Option<Rc<RefCell<ListNode>>> {
        if 0 == list_len {
            panic!("invalid list_len require 1");
        }

        let nodes = (0..list_len)
            .map(|_| Rc::new(RefCell::new(ListNode { next: None })))
            .collect::<Vec<_>>();
        let tail_position = list_len - 1;

        for (i, node) in nodes.iter().enumerate() {
            if let Some(next_node) = nodes.get(i + 1) {
                node.borrow_mut().next = Some(Rc::clone(next_node));
            }
        }

        if let Some(cycle_position) = cycle_position
            && let (Some(tail_node), Some(cycle_to_node)) =
                (nodes.get(tail_position), nodes.get(cycle_position))
        {
            tail_node.borrow_mut().next = Some(Rc::clone(cycle_to_node));
        }

        Some(Rc::clone(&nodes[0]))
    }

    #[test]
    fn no_cycle_test() {
        let expect = false;
        let no_cycle = build_list_with_cycle(None, 4);
        assert_eq!(LinkedListCycle::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(None, 1);
        assert_eq!(LinkedListCycle::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(Some(4), 2);
        assert_eq!(LinkedListCycle::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(Some(2), 2);
        assert_eq!(LinkedListCycle::has_cycle(no_cycle), expect);
    }

    #[test]
    fn no_cycle_test_for_floyds_detection() {
        let expect = false;
        let no_cycle = build_list_with_cycle(None, 4);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection(no_cycle),
            expect
        );

        let no_cycle = build_list_with_cycle(None, 1);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection(no_cycle),
            expect
        );

        let no_cycle = build_list_with_cycle(Some(4), 2);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection(no_cycle),
            expect
        );

        let no_cycle = build_list_with_cycle(Some(2), 2);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection(no_cycle),
            expect
        );
    }

    #[test]
    fn no_cycle_test_for_floyds_detection2() {
        let expect = false;
        let no_cycle = build_list_with_cycle(None, 4);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection2(no_cycle),
            expect
        );

        let no_cycle = build_list_with_cycle(None, 1);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection2(no_cycle),
            expect
        );

        let no_cycle = build_list_with_cycle(Some(4), 2);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection2(no_cycle),
            expect
        );

        let no_cycle = build_list_with_cycle(Some(2), 2);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection2(no_cycle),
            expect
        );
    }

    #[test]
    fn with_cycle_test() {
        let expect = true;
        let with_cycle = build_list_with_cycle(Some(1), 4);
        assert_eq!(LinkedListCycle::has_cycle(with_cycle), expect);

        let with_cycle = build_list_with_cycle(Some(1), 3);
        assert_eq!(LinkedListCycle::has_cycle(with_cycle), expect);

        let with_cycle = build_list_with_cycle(Some(2), 3);
        assert_eq!(LinkedListCycle::has_cycle(with_cycle), expect);
    }

    #[test]
    fn with_cycle_test_for_floyds_detection() {
        let expect = true;
        let with_cycle = build_list_with_cycle(Some(1), 4);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection(with_cycle),
            expect
        );

        let with_cycle = build_list_with_cycle(Some(1), 3);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection(with_cycle),
            expect
        );

        let with_cycle = build_list_with_cycle(Some(2), 3);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection(with_cycle),
            expect
        );
    }

    #[test]
    fn with_cycle_test_for_floyds_detection2() {
        let expect = true;
        let with_cycle = build_list_with_cycle(Some(1), 4);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection2(with_cycle),
            expect
        );

        let with_cycle = build_list_with_cycle(Some(1), 3);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection2(with_cycle),
            expect
        );

        let with_cycle = build_list_with_cycle(Some(2), 3);
        assert_eq!(
            LinkedListCycle::has_cycle_floyds_detection2(with_cycle),
            expect
        );
    }
}
