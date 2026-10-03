use std::collections::HashMap;

struct SubarraySumEqualsK {}
impl SubarraySumEqualsK {
    /*
     * 問題の理解:
     * 整数からなる配列numsと整数kが与えられる。
     * 合計がkに等しくなる部分配列の総数を返す。
     * 部分配列とは配列内の連続した空でない要素のシーケンスのこと。
     * memo:
     * nums[i]を取り出したらnums[i+1..]から補数であるk - nums[i]
     * と等しくなるnums[i+1..]の値を探す感じだと思う。
     * nums[i]を取り出したときに、nums[i+1..]をどこまで伸ばせるかを考える感じだろうか。
     * complement = k - nums[i] として、再帰処理をしながらk = complementをとしてnums = nums[i+1..]
     * のように引き継ぐ。
     * k - nums[i] == 0 のときにreturn 1として、合計がkと等しくなるパスの総数が答えになると思う。
     * 同じ計算を繰り返して、計算量が爆発しないように計算結果をキャッシュする必要がある。
     * 時間をかけすぎすぎたので答えを見る。
     *
     * 解法の理解:
     * nums[i]の累積和を作って、k - prefix_sums[i] が prefix_sums[i]と等しくなればカウントを増やす。
     * 累積和を作って、kを引いた値と同じ累積和存在する、ある区間の合計がkと等しくなるという点が難しく感じる。
     * ポイントとしては累積和のスタートは必ずベースとして0が存在する。
     * ループが進むごとに同じ値の累積和が出現すると頻度を数え上げている。
     * ループ後半で二重にカウントしているように見えるがそうはなっていない。
     * nums=[0,0] k=0, out=3([0],[0],[0,0]) となる例で考えると分かりやすい。
     */
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut prefix_sum_to_frequency: HashMap<i32, i32> = HashMap::new();
        prefix_sum_to_frequency.insert(0, 1);
        let mut prefix_sum = 0;
        for num in nums {
            prefix_sum += num;
            let need = prefix_sum - k;
            if let Some(frequency) = prefix_sum_to_frequency.get(&need) {
                count += frequency;
            };

            prefix_sum_to_frequency
                .entry(prefix_sum)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }

        count
    }
}
