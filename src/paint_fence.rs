struct PaintFence {}
impl PaintFence {
    /*
    LeetCode代替
    https://www.lintcode.com/problem/514/

    問題の理解:
    n本の柱があり、それぞれの柱の色はk色のいずれかで塗装することができる。
    誤り -> -- 隣接する2本の柱の柱が同じ色にならないように、すべての柱を塗装する必要がある。 --
    隣接する柱に同じ色を塗ってよいのは２本までとする。
    柱を塗装する方法の総数を返す。

    memo:
    ある時点の状態が前の状態に依存しているということが考え方のポイントだった気がする。
    問題の理解自体を間違えている -> -- Exampleを見るとpost1, post2が同じ色で塗られているパターンを数えていてよく分からない。隣接する2本の柱は同じ色にならないようにするのでは？ --
    手が止まったので答えを見る。

    解法の理解:
    問題文を日本語訳した時点で意味が変わってしまっていて、正確には連続して同じ色になって良い柱の数は2本までだった。
    今回の問題における動的計画法の考え方
    - base case（初期状態）を考える
    - base case から次の状態への遷移のパターンを考える
        - 直前の柱(i-1)と同じ色を塗る
            - 2本以上連続で同じ色は塗れない
        - 直前の柱(i-1)と違う色を塗る
    - 求める状態(柱の本数)になった時に答えが得られるようにする
    同じ色が続いて良いのは、2本まで。
    柱の状態として取り得るのは、最後の2本(i-1,i)が同じ(same)、または異なる(diff)状態
    柱が0本の時、same[0],diff[0]ともに塗る対象の柱が存在しないので0通り
    柱(i)が1本,色(k)が3色(A,B,C)のとき:
        same[1] = 0
            最後の2本が同じ色である状態は0通り。なぜなら柱が1本しか存在せずi-1の柱は何色でもないので。
        diff[1] = k
            A
            B
            C
            i-1は存在せず、すべての色を使える。つまり与えられた色の数k色利用できる。
    柱(i)が2本,色(k)が3色(A,B,C)のとき:
        same[2]:
            最後の2本が同じ色になる状態はk通り。つまりdiff[i-1]と同じパターン数になる。
            AA
            BB
            CC
        diff[2]:
            AB
            AC
            BA
            BC
            CA
            CB
    状態の遷移からパターンを見つけ出す。
    今回の問題では状態の遷移を漸化式で表すと、
    same[i] = diff[i-1]
    diff[i] = (same[i-1] + diff[i-1]) * (k-1)
    となる。
    最終的な答えはパターンの総数なので、柱の数をi,色の種類をkとすると
    total[i] = same[i] + diff[i]
    となる。

    解き方のパターンを覚える、解けなかった問題の解法を理解するといった練習を繰り返して慣れる必要があるという感覚。
    */

    pub fn num_ways(n: usize, k: usize) -> usize {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return k;
        }

        let mut tail_two_same = vec![0; n];
        let mut tail_two_diff = vec![k; n];

        for i in 1..n {
            tail_two_same[i] = tail_two_diff[i - 1];
            tail_two_diff[i] = (tail_two_same[i - 1] + tail_two_diff[i - 1]) * (k - 1);
        }

        tail_two_same[n - 1] + tail_two_diff[n - 1]
    }

    pub fn num_ways2(n: usize, k: usize) -> usize {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return k;
        }
        let mut previous_tail_two_same = 0;
        let mut previous_tail_two_diff = k;
        for _ in 2..=n {
            let tail_two_same = previous_tail_two_diff;
            let tail_two_diff = (previous_tail_two_same + previous_tail_two_diff) * (k - 1);

            previous_tail_two_same = tail_two_same;
            previous_tail_two_diff = tail_two_diff;
        }

        previous_tail_two_same + previous_tail_two_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_ways_test() {
        assert_eq!(PaintFence::num_ways(3, 2), 6);
        assert_eq!(PaintFence::num_ways(2, 2), 4);
        assert_eq!(PaintFence::num_ways(3, 1), 0);
        assert_eq!(PaintFence::num_ways(4, 2), 10);

        // --- basic cases ---
        assert_eq!(PaintFence::num_ways(1, 1), 1);
        assert_eq!(PaintFence::num_ways(1, 2), 2);
        assert_eq!(PaintFence::num_ways(1, 3), 3);
        assert_eq!(PaintFence::num_ways(2, 1), 1);
        assert_eq!(PaintFence::num_ways(2, 3), 9);

        // --- constraint cases ---
        assert_eq!(PaintFence::num_ways(3, 1), 0);
        assert_eq!(PaintFence::num_ways(4, 1), 0);
        assert_eq!(PaintFence::num_ways(5, 1), 0);
        assert_eq!(PaintFence::num_ways(3, 3), 24);

        // --- larger values ---
        assert_eq!(PaintFence::num_ways(4, 3), 66);
        assert_eq!(PaintFence::num_ways(5, 2), 16);
        assert_eq!(PaintFence::num_ways(5, 3), 180);
    }

    #[test]
    fn num_ways2_test() {
        assert_eq!(PaintFence::num_ways2(3, 2), 6);
        assert_eq!(PaintFence::num_ways2(2, 2), 4);
        assert_eq!(PaintFence::num_ways2(3, 1), 0);
        assert_eq!(PaintFence::num_ways2(4, 2), 10);

        // --- basic cases ---
        assert_eq!(PaintFence::num_ways2(1, 1), 1);
        assert_eq!(PaintFence::num_ways2(1, 2), 2);
        assert_eq!(PaintFence::num_ways2(1, 3), 3);
        assert_eq!(PaintFence::num_ways2(2, 1), 1);
        assert_eq!(PaintFence::num_ways2(2, 3), 9);

        // --- constraint cases ---
        assert_eq!(PaintFence::num_ways2(3, 1), 0);
        assert_eq!(PaintFence::num_ways2(4, 1), 0);
        assert_eq!(PaintFence::num_ways2(5, 1), 0);
        assert_eq!(PaintFence::num_ways2(3, 3), 24);

        // --- larger values ---
        assert_eq!(PaintFence::num_ways2(4, 3), 66);
        assert_eq!(PaintFence::num_ways2(5, 2), 16);
        assert_eq!(PaintFence::num_ways2(5, 3), 180);
    }
}
