use std::{cell::RefCell, collections::HashSet, rc::Rc};

type ListNodeRef = Rc<RefCell<ListNode>>;
struct ListNode {
    next: Option<ListNodeRef>,
}

struct LinkedListCycleII {}
impl LinkedListCycleII {
    /*
    問題の理解:
    連結リストの先頭ノードが与えられるので、サイクルの開始ノードを返す。
    サイクルがない場合はnullを返す。
    サイクルの開始ノードとは、1 -> 2 -> 1のとき、1のノードを返す。

    memo:
    ナイーブな実装として、辿ったノードをHashSetに記録しながら連結リストを走査する。
    重複を検知した時点のノードが分かればそこからサイクルが開始していることが分かるので、そのノードを早期リターンで返す。
    関数終了まで到達したらNoneを返す。
    OK
    */
    fn detect_cycle(head: Option<ListNodeRef>) -> Option<ListNodeRef> {
        let head = head?;
        let mut visited_nodes: HashSet<_> = HashSet::new();
        visited_nodes.insert(Rc::as_ptr(&head));
        let mut current_node = Rc::clone(&head);
        while let Some(node) = &Rc::clone(&current_node).borrow().next {
            if !visited_nodes.insert(Rc::as_ptr(node)) {
                return Some(Rc::clone(node));
            }
            current_node = Rc::clone(node);
        }

        None
    }

    /*
    LLMによるレビュー指摘箇所の修正
    - borrowのスコープを明示的に切る

    memo:
    Rc<T>には.as_ptr()が無いのでT=RefCellの.as_ptr()が呼ばれてしまい想定通りの動きにならないので注意。
    Rc::as_ptr()を使う必要がある。
    IDEでRc<T>に対してas_ptr()が呼べるように見えるが、実際はRc<T>がDref<Target = T>を実装しているのでTであるRefCellのas_ptrが呼び出される。
    */
    fn detect_cycle2(head: Option<ListNodeRef>) -> Option<ListNodeRef> {
        let head = head?;
        let mut visited_nodes = HashSet::new();
        visited_nodes.insert(Rc::as_ptr(&head));
        let mut current_node = Rc::clone(&head);
        loop {
            let next_node = {
                let node = current_node.borrow();
                node.next.as_ref().map(Rc::clone)
            };
            let next_node = next_node?;
            if !visited_nodes.insert(Rc::as_ptr(&next_node)) {
                return Some(Rc::clone(&current_node));
            }

            current_node = next_node;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CycledNodeList {
        head: Option<ListNodeRef>,
        cycle_start_node: Option<ListNodeRef>,
    }

    fn build_list_with_cycle(cycle_position: Option<usize>, list_len: usize) -> CycledNodeList {
        if list_len == 0 {
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

        let head = Some(Rc::clone(&nodes[0]));
        let mut cycle_start_node = None;
        let Some(cycle_position) = cycle_position else {
            return CycledNodeList {
                head,
                cycle_start_node: None,
            };
        };

        if let (Some(tail_node), Some(cycle_to_node)) =
            (nodes.get(tail_position), nodes.get(cycle_position))
        {
            tail_node.borrow_mut().next = Some(Rc::clone(cycle_to_node));
            let _ = cycle_start_node.insert(Rc::clone(cycle_to_node));
        }

        CycledNodeList {
            head,
            cycle_start_node,
        }
    }

    fn node_ref_ptr_eq(this: &Option<ListNodeRef>, other: &Option<ListNodeRef>) -> bool {
        match (this, other) {
            (Some(this), Some(other)) => Rc::ptr_eq(this, other),
            (None, None) => true,
            _ => false,
        }
    }

    #[test]
    fn no_cycle_test() {
        let expect = None;

        let cycled_node_list = build_list_with_cycle(None, 1);
        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &expect
        ));

        let cycled_node_list = build_list_with_cycle(Some(3), 2);
        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &expect
        ));

        let cycled_node_list = build_list_with_cycle(Some(2), 2);
        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &expect
        ));
    }

    #[test]
    fn cycle_test() {
        let cycle_position = Some(7);
        let cycled_node_list = build_list_with_cycle(cycle_position, 8);
        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head.as_ref().map(Rc::clone)),
            &cycled_node_list.cycle_start_node
        ));

        let cycle_position = Some(0);
        let cycled_node_list = build_list_with_cycle(cycle_position, 2);
        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &cycled_node_list.cycle_start_node
        ));

        let expect_cycle_position = Some(0);
        let cycled_node_list = build_list_with_cycle(expect_cycle_position, 1);
        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &cycled_node_list.cycle_start_node
        ));
    }

    // // 以下、ChatGPT(GPT-5)で生成
    #[test]
    #[should_panic(expected = "invalid list_len require 1")]
    fn should_panic_on_zero_length() {
        let _ = build_list_with_cycle(None, 0);
    }

    #[test]
    fn cycle_at_tail_self_loop() {
        let list_len = 5;
        let cycled_node_list = build_list_with_cycle(Some(list_len - 1), list_len);

        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &cycled_node_list.cycle_start_node
        ));
    }

    #[test]
    fn cycle_in_middle() {
        let list_len = 10;
        let cycled_node_list = build_list_with_cycle(Some(4), list_len);

        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &cycled_node_list.cycle_start_node
        ));
    }

    #[test]
    fn two_nodes_cycle_to_tail() {
        let cycled_node_list = build_list_with_cycle(Some(1), 2);

        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &cycled_node_list.cycle_start_node
        ));
    }

    #[test]
    fn long_list_no_cycle() {
        let cycled_node_list = build_list_with_cycle(None, 1000);

        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &cycled_node_list.cycle_start_node
        ));
    }

    #[test]
    fn long_list_cycle_early() {
        let list_len = 200;
        let cycled_node_list = build_list_with_cycle(Some(3), list_len);

        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &cycled_node_list.cycle_start_node
        ));
    }

    #[test]
    fn long_list_cycle_late() {
        let list_len = 200;
        let cycled_node_list = build_list_with_cycle(Some(198), list_len);

        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &cycled_node_list.cycle_start_node
        ));
    }

    #[test]
    fn out_of_range_cycle_position_means_no_cycle() {
        let cycled_node_list = build_list_with_cycle(Some(9999), 5);

        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &None
        ));

        let cycled_node_list = build_list_with_cycle(Some(10), 10);

        assert!(node_ref_ptr_eq(
            &LinkedListCycleII::detect_cycle(cycled_node_list.head),
            &None
        ));
    }
}
