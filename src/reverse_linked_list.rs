struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

struct ReverseLinkedList {}
impl ReverseLinkedList {
    /*
    単方向連結リストが引数で与えられるので、逆順にした単方向連結リストを返す。
    ノードの値を先頭から全て取り出す。
    取り出したノードの値の配列を使って逆順になるように単方向連結リストを生成して返す。
    解けた。
    foldで単方向連結リストを生成する書き方を覚えてしまい、この書き方に固定されている感じが良くない気がする。
    考えながら書いていると言うよりはこう書いたらこうなると流れで書いている感じはある。
    */
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node_values = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            node_values.push(node.val);
            current = node.next;
        }

        node_values.into_iter().fold(None, |child, node_value| {
            let mut parent = Box::new(ListNode::new(node_value));
            parent.next = child;
            Some(parent)
        })
    }

    /*
    LeetCodeのFollow upで再帰または反復で書き直せるかとあるので、再帰を使ったバージョンを書いてみる
    思いつかなかったのでLLMに聞いて写経

    コードの理解:
    再帰処理になっている。
    base case: 今見ている(current)がNoneであれば、末尾にいるので1つ前のノード(previous)を返す。
    recursive case:
    逆順にしたいという気持ちがある。
    - 次のノードを取り出しておく。直後に次のノードとして、1つ前のノードを設定するため。
    - 1つ前のノードを次のノードとして設定する。
    - 今見ているノードを前のノードとして次に引き継ぐ
    - ノードの辿り方は普通に考えれば良い
     - node.nextを順に辿っていく
    単方向連結リストを順に見ていくという操作とこの操作に影響を与えないように前後のノードを入れ替えられるか？をそれぞれ独立させて考えるとよそさう。
    */
    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(
            previous: Option<Box<ListNode>>,
            current: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match current {
                None => previous,
                Some(mut node) => {
                    let next_node = node.next.take();
                    node.next = previous;
                    reverse(Some(node), next_node)
                }
            }
        }

        reverse(None, head)
    }

    // 再帰をそのまま反復で書き直す
    pub fn reverse_list_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = head;
        while let Some(mut node) = current {
            let next_node = node.next;
            node.next = previous;
            previous = Some(node);
            current = next_node;
        }

        previous
    }
}
