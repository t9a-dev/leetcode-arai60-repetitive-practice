use std::collections::HashMap;

struct TwoSum {}

impl TwoSum {
    /*
    問題の理解:
    整数からなる配列numsと整数targetが与えられる。
    numsの要素の内、2つの整数の和がtargetと等しくなるnumsの要素のindexを配列で返す。
    配列内の並び順は任意。
    同じ要素を二回使うことはできない。

    memo:
    num[i]をHashMapのキー、値をnumsにおけるindexとする
    numsの値を先頭から見ながらtargetとの差を求めて、HashMapから差をキーとして対応するindexを求める。
    配列のindexを返せと問題にあるのに戻り値のシグネチャがi32の配列なのは止めて欲しいなと思った。usizeの配列にしてほしい。

    3回WrongAnswerとなった。解けたとは言えないのでNGとしておく。

    */
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let num_to_index: HashMap<_, _> =
            HashMap::from_iter(nums.iter().enumerate().map(|(i, num)| (num, i)));

        for (i, num) in nums.iter().enumerate() {
            let diffirence_from_target = target - num;
            if let Some(j) = num_to_index.get(&diffirence_from_target) {
                if i == *j {
                    continue;
                }

                return vec![i as i32, *j as i32];
            }
        }

        panic!("two sum pair not found.")
    }

    /*
    ナイーブな実装O(n ^ 2)も実装しておく
    nums.iter().skip(i + 1).enumerate() としてWrongAnswerとなった。
    nums.iter().enumerate().skip(i + 1) が正しい。
    先にskip()してからenumrate()すると、同じサイズの配列を見てないのでインデックスがずれる。
    同じサイズの配列を扱いながらskip(i + 1)とするのが正しい。
    */
    pub fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num_i) in nums.iter().enumerate() {
            for (j, num_j) in nums.iter().enumerate().skip(i + 1) {
                if num_i + num_j == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        panic!("two sum pair not found.")
    }
}
