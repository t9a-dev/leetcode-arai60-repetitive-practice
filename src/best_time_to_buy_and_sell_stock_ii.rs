struct BestTimeToBuyAndSellStockII {}
impl BestTimeToBuyAndSellStockII {
    /*
    問題の理解:
    整数からなる配列pricesが与えられる。prices[i]はi日における株の価格を表している。
    各日において、株式を購入又は売却するかどうかを自由に決定できる。ただし、一度に保有できる株式は1つのみ。
    同じ日に複数回売買を行うことは可能。
    株式売買による最大利益を計算して返す。

    memo:
    prices[0]の株式を持った状態からスタートする。
    持っているより安い株を見つけたら交換する。出力例を見ると安い株に交換する時に損失が発生していないように見えるのが謎だが気にする必要なさそう。
    持っている株より高い株を見つけたら、差を利益として計上する。
    これを繰り返した発生した利益の合計を返す。

    Accepted
    */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut total_profit = 0;
        let mut hold_stock = prices[0];
        for &price in prices.iter().skip(1) {
            if price < hold_stock {
                hold_stock = price;
            } else {
                total_profit += price - hold_stock;
                hold_stock = price;
            }
        }

        total_profit
    }
}
