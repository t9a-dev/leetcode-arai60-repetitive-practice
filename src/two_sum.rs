use std::collections::HashMap;

struct TwoSum {}
impl TwoSum {
    /*
    問題の理解:
    整数からなる配列numsと整数targetが与えられる。
    nums要素の2つの合計値がtargetと等しくなるような値のインデックスを返す。
    答えは必ず1つ存在する。
    同じ要素を2回使ってはならない。

    memo:
    target - nums[i]との値をkeyとしてiをvalueとするHashMapを作る。
    numsを走査しながらnums[i]をHashMapからlookupして該当するインデックスがあればそれが答え。
    同じ要素は2回使えないので同じindexではないことを確認して、indexを配列にして返す。
    Accepted
    */
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let complement_to_index: HashMap<i32, usize> =
            HashMap::from_iter(nums.iter().enumerate().map(|(i, v)| (target - v, i)));
        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = complement_to_index.get(num) {
                if i == j {
                    continue;
                }
                return vec![i as i32, j as i32];
            }
        }
        unreachable!("問題の制約上、答えとなるペアが必ず存在するためここに到達しない")
    }
}
