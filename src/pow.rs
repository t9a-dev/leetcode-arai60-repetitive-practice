struct Pow {}
impl Pow {
    /*
    問題の理解:
    64ビット浮動小数点xと整数nが与えられる。
    xをn乗して答えを返す。
    問題の制約として、f64をオーバーフローするような入力の組み合わせは与えられない

    memo:
    nが正数であればn回乗算、負数であればn回除算すれば良さそう。

    Wrong Answerとなった。 n = 0 つまり、xの0乗が1になることを忘れていた。
    Time Limit Exceededとなった。x = 0.00001, n = 2147483647

    答えを見る。

    解法の理解:
    再帰でn / 2にして計算して、この答え同士を乗算することで時間計算量O(log n)にしている。
    2 ^ 8 = (2 ^ 4) * (2 ^ 4)
    2 ^ 4 = (2 ^ 2) * (2 ^ 2)
    0乗が1になるのは覚えるとして、ナイーブな計算をすると、時間計算量がO(n)になってTime Limit Exceededとなるのは気付ける部分だと思った。
    */

    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        // n = 2 ^ 31 ~ 2 ^ 31 - 1 となり、i32::MINを反転した時にオーバーフローするのでi64とする。
        let mut n = n as i64;
        if n < 0 {
            x = 1.0 / x;
            n = n.abs();
        }

        let half = Self::my_pow(x, (n / 2) as i32);
        if n % 2 == 0 {
            return half * half;
        }

        x * half * half
    }

    pub fn wrong_answer_my_pow(x: f64, n: i32) -> f64 {
        /*
        Wrong Answer
        */
        if n == 0 {
            return 1.0;
        }

        let mut result = x;
        for _ in 0..n.abs() - 1 {
            result *= x;
        }

        if 0 < n {
            return result;
        }
        1.0 / result
    }
}
