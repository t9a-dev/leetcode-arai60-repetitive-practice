/*
  https://neetcode.io/problems/linked-list-cycle-detection/question
  - 問題の理解
  連結リストの先頭要素が与えられる。連結リストに循環が存在する時trueを返す。循環が存在しなければfalseを返す。
  循環とはあるノードの次のノードを辿っていった時に、同じノードに再びアクセスできること。

  memo:
  - ノードが持つ値自体を管理すると、重複するノードの値が存在する場合に正しく検知できない。
  - ノードのアドレス位置をsetで管理しておいて、重複検知するのがナイーブな実装として思いつく。
  - *mut ListNode型のheadからnextを参照する方法がわからず、問題が解けなかった。
    - NeetCodeのSolutionではシグネチャがOption<Box<ListNode>>に変更されている上で、２ポインタによるフロイドの循環検出で解くようにとある。
    - unsafeコードを書けず解けなかった。
  - このシグネチャだとunsafeコードを書く必要がある様子。写経だけしておく。
  - unsafeコードで解ける以上は、書けるようにしておきたい。
    - ポインタとunsafeコード周りの理解ができていれば、自然と解けるように見える。
    - 『プログラミングRust 第2版　第22章　unsafeなコード』を読むと良さそう。
      - 『22.8 rawポインタ』に、ここで利用している参照解決(*tail)についての説明がある。
      参照解決(*tail)をunsafeブロック内で行う必要がある。
  - unsafeコードを扱う良い練習になった。
*/

use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: *mut ListNode,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: std::ptr::null_mut(),
        }
    }
}

pub struct LinkedListCycle {}
impl LinkedListCycle {
    pub fn has_cycle_set(head: *mut ListNode) -> bool {
        if head.is_null() {
            return false;
        }

        let mut tail = head as *const ListNode;
        let mut visited_nodes = HashSet::new();
        visited_nodes.insert(tail);

        unsafe {
            while !tail.is_null() && !(*tail).next.is_null() {
                if !visited_nodes.insert((*tail).next) {
                    return true;
                }
                tail = (*tail).next;
            }
        }

        false
    }

    pub fn has_cycle_floyd(head: *mut ListNode) -> bool {
        if head.is_null() {
            return false;
        }

        let mut slow = head;
        let mut fast = head;

        unsafe {
            while !fast.is_null() && !(*fast).next.is_null() {
                slow = (*slow).next;
                fast = (*(*fast).next).next;

                if slow == fast {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null_mut;

    fn make_linked_list(nums: Vec<i32>, linke_to_index: Option<usize>) -> *mut ListNode {
        if nums.is_empty() {
            return null_mut();
        }

        let mut head: *mut ListNode = null_mut();
        let mut tail: *mut ListNode = null_mut();
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

        let Some(link_to_index) = linke_to_index else {
            return head;
        };
        let Some(link_to_node) = nodes.get(link_to_index) else {
            return head;
        };

        unsafe {
            (*tail).next = *link_to_node;
        }

        head
    }

    #[test]
    pub fn not_cycled_test() {
        let head = make_linked_list(vec![1, 2], None);
        assert_eq!(LinkedListCycle::has_cycle_set(head), false);

        let head = make_linked_list(vec![1], None);
        assert_eq!(LinkedListCycle::has_cycle_set(head), false);
    }

    #[test]
    pub fn cycled_test() {
        let head = make_linked_list(vec![1, 2, 3, 4], Some(1));
        assert_eq!(LinkedListCycle::has_cycle_set(head), true);
    }
}
