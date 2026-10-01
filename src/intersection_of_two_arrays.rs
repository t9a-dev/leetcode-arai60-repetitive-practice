use std::collections::HashSet;

struct IntersectionOfTwoArrays {}
impl IntersectionOfTwoArrays {
    /*
     * 問題の理解:
     * 2つの整数からなる配列nums1,nums2が与えられたとき、それらの共通部分となる配列を返す。
     * 答えの配列の要素はそれぞれ一意である必要があり、結果の順序は任意。
     *
     * memo:
     * intersectionメソッドを使えば解ける問題ではある。
     * nums1,nums2両方に存在する値のみを取り出す必要がある。
     * 片方の配列からHashSetを作る。
     * もう一方の配列を操作しながらHashSetに同じ値が存在するか確認する。HashSetに対するlookupなので高速(O(1))に行える。
     * 答えを保持するHashSetに見つけた値を入れていく。
     * 配列にして返す。
     * Accepted
     */
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set: HashSet<_> = HashSet::from_iter(nums1);
        let mut intersected = HashSet::new();
        for num2 in nums2 {
            let Some(v) = nums1_set.get(&num2) else {
                continue;
            };
            intersected.insert(*v);
        }
        intersected.into_iter().collect()
    }

    /*
     * 過去に書いたコードを眺めていて気付いた箇所をリファクタリング
     * - HashSetにするのはサイズの小さい配列を使う。
     * - HashSet.get()を使う必要はなくて、HashSet.contains()で良い。
     * - nums1, nums2 ともに引数の型が所有権を受け取るシグネチャなのでmutableにしても呼び出し側に影響はない。
     */
    pub fn intersection2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let base_nums_set: HashSet<_> = HashSet::from_iter(nums1);
        let mut intersected = HashSet::new();
        for num2 in nums2 {
            if !base_nums_set.contains(&num2) {
                continue;
            }
            intersected.insert(num2);
        }

        intersected.into_iter().collect()
    }
}
