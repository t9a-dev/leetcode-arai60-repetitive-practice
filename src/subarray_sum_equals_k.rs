use std::collections::HashMap;

struct SubArraySumEqualsK {}

impl SubArraySumEqualsK {
    /*
    整数からなる配列nums,整数kが与えられる。
    整数kと合計値が等しい、numsの部分配列の数を返す。

    memo:
    numsから作れる部分配列を網羅的に作るのかが分からず手が止まったので答えを見て写経する。

    解法の理解:
    累積和とその出現頻度をHashMapで管理している。
    累積和とkの差である補数を計算。
    補数と等しい累積和がHashMapにあれば、その累積和の出現回数を答えのcountに足す。
    累積和,累積和の出現回数のHashMapに追加。

    累積和を3としたとき、[1,2],[3]のように2つの部分列がある。
    HashMapでは、ある累積和を作れる部分配列の数（出現頻度）だけを記録しておいて、kとの差(補数)に等しい累積和を確認することで、
    kと等しくなるような部分配列の総数を計算している。

    complement = k - prefix_sumとしてWrong Answerとなった。
    現在のprefix_sum - 過去のprefix_sum = k
    過去のprefix_sum = 現在のprefix_sum - k
    そもそも、complement（補数）という変数名が良くなさそう。
    LLMに相談してneeded_prefix_sumとすることにした。
    */
    pub fn subarray_sum_wrong_answer(nums: Vec<i32>, k: i32) -> i32 {
        /////////////////
        // Wrong Answer
        /////////////////
        let mut count = 0;
        let mut prefix_sum = 0;
        let mut prefix_sum_to_frequency: HashMap<_, usize> = HashMap::from_iter([(0, 1)]);
        for num in nums {
            prefix_sum += num;
            let complement = k - prefix_sum; // BUG
            if let Some(frequency) = prefix_sum_to_frequency.get(&complement) {
                count += frequency;
            }

            prefix_sum_to_frequency
                .entry(prefix_sum)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }

        count as i32
    }

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut prefix_sum = 0;
        let mut prefix_sum_to_frequency: HashMap<_, usize> = HashMap::from_iter([(0, 1)]);
        for num in nums {
            prefix_sum += num;
            let needed_prefix_sum = prefix_sum - k;
            if let Some(frequency) = prefix_sum_to_frequency.get(&needed_prefix_sum) {
                count += frequency;
            }

            prefix_sum_to_frequency
                .entry(prefix_sum)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }

        count as i32
    }
}
