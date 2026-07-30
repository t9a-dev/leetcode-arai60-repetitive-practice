use std::collections::HashMap;

struct CoinChange {}
impl CoinChange {
    /*
    問題の理解:
    整数からなる配列coins,amount が与えられる。
    coins[i]は異なる金額のコインを表している。
    amountは合計金額を表している。
    coinsに含まれるコインからamountになる組み合わせのうち、最小の枚数を返す。
    coinsに含まれる各金額のコインは何枚でも利用して良い。
    coinsからamountを作ることができないときは-1を返す。

    memo:
    求めたいのはamountになるコインの最小枚数。
    coinsのうち大きい金額のコインから見ていったほうが良い。少ないコインの枚数を知りたいので。
    dpテーブルを使うような動的計画法がすぐに思いつかないので、決定木で考えて再帰で解く方向でやる。
    amountをスタートとして、coins[i]で減算しながら0と等しくなればそこまでに使ったコインの数を返す。
    ある時点のamountとコインの枚数をキャッシュする。
    再帰に入るたびにコインの枚数を加算する。
    base case:
    0未満は作れないので再帰に入らない
    0を超える値で再帰に入る
    0と等しければ終了

    Wrong Answerとなった。キャッシュの仕方が間違っている気がする。
    キャッシュの位置を修正したがWrong Answerとなったので、答えを見る。

    解法の理解:
    amountを作るために必要な最小コイン枚数を求めるという部分問題として考える。
    */
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut amount_to_coin_count = HashMap::new();
        let mut coins = coins.clone();
        coins.sort();
        Self::min_coin_count(&coins, amount, &mut amount_to_coin_count).unwrap_or(-1)
    }

    fn min_coin_count(
        coins: &[i32],
        amount: i32,
        amount_to_coin_count: &mut HashMap<i32, Option<i32>>,
    ) -> Option<i32> {
        if amount == 0 {
            return Some(0);
        }
        if amount < 0 {
            return None;
        }
        if let Some(&coin_count) = amount_to_coin_count.get(&amount) {
            return coin_count;
        }

        let min_coin_count = coins
            .iter()
            .flat_map(|&coin| {
                let remaining_amount = amount - coin;
                let coin_count =
                    Self::min_coin_count(coins, remaining_amount, amount_to_coin_count)?;
                Some(coin_count + 1)
            })
            .min();
        amount_to_coin_count.insert(amount, min_coin_count);

        min_coin_count
    }

    /*
    メモ化再帰による実装をDPテーブルに書き換える練習。
    amountが0のとき、最小コイン枚数は0枚であることをbase caseとする。
    dp[0] = 0;
    amount = 1, coins = [1, 3, 4]
    coins[i] <= amount のとき、
    dp[1] = dp[amount - coins[i]] + 1 = dp[1 - 1] + 1 = dp[0] + 1 となる。
    番兵値 amount + 1 について、coinsが取り得る値の最小値は1かつ整数なので、すべて1のコインを使った時にamountと同じ枚数になる。
    なので、amount + 1 は到達不可能なコイン枚数の番兵値として扱うことができる。
    */
    pub fn coin_change2(coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 0 {
            return -1;
        }
        let sentinel_coin_count = amount as usize + 1;
        let mut amount_to_min_coin_count = vec![sentinel_coin_count; sentinel_coin_count];
        amount_to_min_coin_count[0] = 0;

        for coin in coins {
            for a in 1..=amount {
                if a < coin {
                    continue;
                }
                let remaining_amount = a - coin;
                amount_to_min_coin_count[a as usize] = amount_to_min_coin_count[a as usize]
                    .min(amount_to_min_coin_count[remaining_amount as usize] + 1);
            }
        }

        let min_coin_count = amount_to_min_coin_count[amount as usize];
        if min_coin_count == sentinel_coin_count {
            return -1;
        }

        min_coin_count as i32
    }
}
