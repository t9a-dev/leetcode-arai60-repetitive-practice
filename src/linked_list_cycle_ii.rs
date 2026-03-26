/*
  https://leetcode.com/problems/linked-list-cycle-ii/description/
  - 問題の理解
  連結リストの先頭要素が与えられる。連結リストに循環が存在する時、循環の開始ノードを返す。存在しない場合はnullを返す。
  循環が存在する時、tailノードが接続している先のノードを返せば良さそう。

  memo:
  - 採点システムがRustに対応していないので、LLM使いつつ自前で実装する。
    - Rc<RefCell<ListNode>>ではなくて、rawポインタ(*mut ListNode)を扱うunsafeコードで解く。1周目ではunsafeコードを書いていなかったため。
  - LinkedListInfoのcycle_to_nodeをOption<*mut ListNode>で表そうかと思ったが、Option型にしたところで、ポインタがnullでないことは保証できないことに気付いて止めた。
    - Some()で取り出した値がnullである可能性を覆い隠してしまう感覚があって良くないと思った。
  - HashSetを用いた重複検知のコードは素直に書けた。
  - フロイドの循環検出法は相変わらず、こうなるからこうという感じで書いている。
    - 2ポインタ(slow,fast)で循環を検出して、検出時に見ているノードを返す。
    - 2ポインタ(head,cycled)をそれぞれ、1つずつ次のノードに進みながら見ていく。
    - head == cycledとなったときのノードが、循環の開始ノードになっている。
*/

use std::{collections::HashSet, ptr::null_mut};

#[allow(unused)]
pub struct ListNode {
    val: i32,
    next: *mut ListNode,
}
impl ListNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: null_mut(),
        }
    }
}

pub struct LinkedListCycleII {}
impl LinkedListCycleII {
    pub fn detect_cycle(head: *mut ListNode) -> *mut ListNode {
        if head.is_null() {
            return null_mut();
        }

        let mut tail = head;
        let mut visited = HashSet::new();

        unsafe {
            while !tail.is_null() {
                if !visited.insert(tail) {
                    return tail;
                }
                tail = (*tail).next;
            }
        }

        null_mut()
    }

    pub fn detect_cycle_floyd(head: *mut ListNode) -> *mut ListNode {
        if head.is_null() {
            return null_mut();
        }

        let mut cycled_node = Self::find_cycle_node(head);
        if cycled_node.is_null() {
            return null_mut();
        }

        let mut node = head;
        unsafe {
            while !node.is_null() && !cycled_node.is_null() {
                if node == cycled_node {
                    return node;
                }

                node = (*node).next;
                cycled_node = (*cycled_node).next;
            }
        }

        unreachable!()
    }

    fn find_cycle_node(head: *mut ListNode) -> *mut ListNode {
        let mut slow = head;
        let mut fast = head;

        unsafe {
            while !fast.is_null() && !(*fast).next.is_null() {
                slow = (*slow).next;
                fast = (*(*fast).next).next;

                if slow == fast {
                    return fast;
                }
            }
        }
        null_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LinkedListInfo {
        head: *mut ListNode,
        cycle_to_node: *mut ListNode,
    }

    fn make_linked_list(nums: Vec<i32>, link_to_index: Option<usize>) -> LinkedListInfo {
        if nums.is_empty() {
            panic!("nums must not be empty.")
        }

        let mut head: *mut ListNode = null_mut();
        let mut tail = head;
        let mut nodes = Vec::new();

        for num in nums {
            let node = Box::into_raw(Box::new(ListNode::new(num)));
            nodes.push(node);

            if head.is_null() {
                head = node;
                tail = node;
                continue;
            }

            unsafe {
                (*tail).next = node;
            }
            tail = node;
        }

        let Some(link_to_node) = link_to_index.and_then(|i| nodes.get(i)) else {
            return LinkedListInfo {
                head,
                cycle_to_node: null_mut(),
            };
        };

        unsafe {
            (*tail).next = *link_to_node;
        }

        LinkedListInfo {
            head,
            cycle_to_node: *link_to_node,
        }
    }

    #[test]
    fn make_linked_list_cycled_test() {
        let linked_list_info = make_linked_list(vec![3, 2, 0, -4], Some(1));
        unsafe {
            let cycled_node_val = (*linked_list_info.cycle_to_node).val;
            assert_eq!(cycled_node_val, 2);
        }

        let linked_list_info = make_linked_list(vec![3, 2, 0, -4], Some(3));
        unsafe {
            let cycled_node_val = (*linked_list_info.cycle_to_node).val;
            assert_eq!(cycled_node_val, -4);
        }
    }

    #[test]
    fn make_linked_list_no_cycled_test() {
        let linked_list_info = make_linked_list(vec![3, 2, 0, -4], None);
        let cycled_node = linked_list_info.cycle_to_node;
        assert_eq!(cycled_node, null_mut());

        let linked_list_info = make_linked_list(vec![3, 2, 0, -4], Some(10));
        let cycled_node = linked_list_info.cycle_to_node;
        assert_eq!(cycled_node, null_mut());
    }

    #[test]
    fn not_cycled_test() {
        let linked_list_info = make_linked_list(vec![3, 2, 0, -4], None);
        let head = linked_list_info.head;
        assert_eq!(
            LinkedListCycleII::detect_cycle(head),
            linked_list_info.cycle_to_node
        );
    }

    #[test]
    fn cycled_test() {
        let linked_list_info = make_linked_list(vec![3, 2, 0, -4], Some(1));
        let head = linked_list_info.head;
        assert_eq!(
            LinkedListCycleII::detect_cycle(head),
            linked_list_info.cycle_to_node
        );
    }

    #[test]
    fn not_cycled_floyd_test() {
        let linked_list_info = make_linked_list(vec![3, 2, 0, -4], None);
        let head = linked_list_info.head;
        assert_eq!(
            LinkedListCycleII::detect_cycle_floyd(head),
            linked_list_info.cycle_to_node
        );
    }

    #[test]
    fn cycled_floyd_test() {
        let linked_list_info = make_linked_list(vec![3, 2, 0, -4], Some(1));
        let head = linked_list_info.head;
        assert_eq!(
            LinkedListCycleII::detect_cycle_floyd(head),
            linked_list_info.cycle_to_node
        );
    }
}
