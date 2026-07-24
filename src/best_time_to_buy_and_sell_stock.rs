struct BestTimeToBuyAndSellStock {}
impl BestTimeToBuyAndSellStock {
    /*
    問題の理解:
    整数からなる配列pricesが与えられる。
    prices[i]はi日における株式の価格である。
    ある日iを選んで株式を購入して最大の利益を得たときの利益を返す。
    利益が得られないときは0を返す。

    memo:
    pricesのサイズが10 ^ 5なので、O(n ^ 2)となるロジックでは分単位で時間がかかるので、Time Limit Exceededとなる。
    10 ^ 10 / 10 ^ 8 = 100となり、約100秒という見積もり。
    最小値を保持しておく。
    最小値 < prices[i]となるときに利益を計算する
    利益も最大値を保持しておく。

    Acceptedとなったが、1時間くらい悩んだのでNGとしておく。

    過去の回答を見たが、再帰にする必要も感じない。（スタックオーバーフローを気にしながら再帰にする必要を感じない）
    */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut min_price = prices[0];
        let mut max_profit = 0;
        for price in prices.iter().skip(1) {
            min_price = min_price.min(*price);
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }
}
