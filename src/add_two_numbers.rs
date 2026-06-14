pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

struct AddTwoNumbers {}
impl AddTwoNumbers {
    /*
    2つのリンクリストが与えられる。これらのリンクリストは各桁を逆順にした非負数の値を表している。3->2->1 のとき 整数123を表している。
    これら2つの整数を加算し、この結果を返す。
    l1 = [2,4,3] l2 = [5, 6, 4]のとき output = [7, 0, 8] のリンクリストを返す。

    memo:
    ノードの値同士を足した10の桁を次のノードに加算する必要がある
    l1, l2の長さは異なる場合があるので、あるノードと加算するノードが存在しないときは扱う
    両方のリンクリストからノードを取り出せなくなるまでループする必要がある。
    状態の取り回しが面倒そうなので、一度配列に分解して計算した後にListNodeを構築するほうが良いかも。

    一応ナイーブな実装で自力で解けたが、if l1_values.len() < l2_values.len() とするところで.len()をつけ忘れて、ここのバグを取るのに時間がかかった。
    */
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_values = Self::collect_node_values(l1);
        let mut l2_values = Self::collect_node_values(l2);
        if l1_values.len() < l2_values.len() {
            l1_values.resize(l2_values.len(), 0);
        } else {
            l2_values.resize(l1_values.len(), 0);
        }

        let mut carry_list = Vec::new();
        let mut sum_values = l1_values
            .into_iter()
            .zip(l2_values)
            .map(|(l1_value, l2_value)| {
                let carry = carry_list.pop().unwrap_or(0);
                let sum = l1_value + l2_value + carry;
                carry_list.push(sum / 10);
                sum % 10
            })
            .collect::<Vec<_>>();
        sum_values.extend(carry_list.into_iter().filter(|c| 0 < *c));

        sum_values
            .into_iter()
            .rev()
            .fold(None, |child, node_value| {
                let mut parent = Box::new(ListNode::new(node_value));
                parent.next = child;
                Some(parent)
            })
    }

    fn collect_node_values(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut node_values = Vec::new();
        let mut current = head;

        while let Some(node) = current {
            node_values.push(node.val);
            current = node.next;
        }

        node_values
    }
}

struct AddTwoNumbers2 {}
impl AddTwoNumbers2 {
    /*
    別の実装を写経
    */
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_current = l1;
        let mut l2_current = l2;
        let mut dummy_head = Self::create_list_node(0, None);
        let mut tail = dummy_head.as_mut();
        let mut carry = 0;

        while (carry != 0) | l1_current.is_some() | l2_current.is_some() {
            let l1_node = l1_current.unwrap_or(Self::create_list_node(0, None));
            let l2_node = l2_current.unwrap_or(Self::create_list_node(0, None));
            let sum = l1_node.val + l2_node.val + carry;
            let node_value = sum % 10;

            carry = sum / 10;
            tail = tail.next.insert(Self::create_list_node(node_value, None));
            l1_current = l1_node.next;
            l2_current = l2_node.next;
        }

        dummy_head.next
    }

    // LeetCodeの採点システムを無視するのであれば、impl ListNodeでcreate_list_nodeを定義したい
    // ListNode::new(val,next)が一番良いと思う
    fn create_list_node(val: i32, next: Option<Box<ListNode>>) -> Box<ListNode> {
        let mut node = Box::new(ListNode::new(val));
        node.next = next;
        node
    }
}

#[cfg(test)]
mod tests {
    use crate::add_two_numbers::{AddTwoNumbers, ListNode};

    fn build_list_node(values: Vec<i32>) -> Option<Box<ListNode>> {
        values.into_iter().rev().fold(None, |child, node_value| {
            let mut parent = Box::new(ListNode::new(node_value));
            parent.next = child;
            Some(parent)
        })
    }

    fn collect_node_values(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut node_values = Vec::new();
        let mut current = head;

        while let Some(node) = current {
            node_values.push(node.val);
            current = node.next;
        }

        node_values
    }

    #[test]
    fn playground() {
        let a = vec![1, 2, 3];
        let mut b = vec![1];
        b.resize(a.len(), 0);

        let result = a.into_iter().zip(b).map(|(a, b)| a + b).collect::<Vec<_>>();

        assert_eq!(result, vec![2, 2, 3]);
    }

    #[test]
    fn add_two_numbers_test() {
        let sum_head = AddTwoNumbers::add_two_numbers(
            build_list_node(vec![5, 6]),
            build_list_node(vec![5, 4, 9]),
        );

        assert_eq!(collect_node_values(sum_head), vec![0, 1, 0, 1]);
    }
}
