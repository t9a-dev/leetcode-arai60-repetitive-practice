use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: i32,
    scores: BinaryHeap<Reverse<i32>>,
}

/*
KthLargest初期化時に指定されたk位のスコアをaddの戻り値として返す必要がある。
addではスコアのリストに指定されたスコアを追加し、このスコアを含めた上位k位のスコアを戻り値とする。
スコアは降順としたときの上からk位
最小ヒープにスコアを突っ込む。
上位k位未満の値は必要ないので捨てる。
最小ヒープのサイズがkになるまでpopし続ける。peekで上位k位の値が見える状態を維持する。
add実行時は、ヒープへ値のpush,ヒープのサイズがkになるまでpopし続ける,peekした値を返す。

newによる初期化時にnums.len() < k をエラーとするifを書いていたのが原因でWrong Answerとなった。
よくよく考えると、あとから値を追加するので初期化時点で nums.len() < k でも問題はないと気付いた。
この条件文を消してAccepted
*/
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = Self {
            k,
            scores: BinaryHeap::new(),
        };
        for num in nums {
            kth_largest.add(num);
        }

        kth_largest
    }

    /// 新しいスコアを追加し、k位の得点を返す
    fn add(&mut self, val: i32) -> i32 {
        self.scores.push(Reverse(val));
        while self.k < self.scores.len() as i32 {
            self.scores.pop();
        }

        self.scores.peek().unwrap().0
    }
}
