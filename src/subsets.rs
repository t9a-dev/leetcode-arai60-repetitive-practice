struct Subsets {}
impl Subsets {
    /*
    問題の理解:
    一意の要素含む整数配列numsが与えられる。
    全ての可能な部分集合を返す。部分集合とは配列の要素から選ばれた配列の集合のこと。空の配列も含まれる。
    答えは任意の並び順で返して良い。

    memo:
    Setで重複した集合を排除しながら解くのがナイーブな実装に思える。
    再帰で1つの要素を取り除いた配列をHashSetに入れていく。
    決定木で考えたときに、この手順だと重複する部分配列が発生するがHashSetにより重複排除できるという想定。
    時間切れなので答えを見る。


    解法の理解:
    subset.push(nums[i])とした後にsubset.pop()としているので、その関数フレームでsubsetに対して行った変更を元に戻している。
    nums[i + 1..]で一度使った要素を取り除くことで重複した部分配列が作られないようになっている。

    所感:
    再帰処理を見た時に関数フレームごとに状態を考えるようにしたほうが良さそう。
    再帰処理を愚直に追っていくと意味がわからなくなりがち。
    */
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut all_subsets = Vec::new();
        let mut subset = Vec::new();
        Self::make_subsets(&nums, &mut subset, &mut all_subsets);

        all_subsets
    }

    fn make_subsets(nums: &[i32], subset: &mut Vec<i32>, all_subsets: &mut Vec<Vec<i32>>) {
        all_subsets.push(subset.to_vec());

        for i in 0..nums.len() {
            subset.push(nums[i]);
            Self::make_subsets(&nums[i + 1..], subset, all_subsets);
            subset.pop();
        }
    }
}
