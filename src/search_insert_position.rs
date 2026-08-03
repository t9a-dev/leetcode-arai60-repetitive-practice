struct SearchInsertPosition {}
impl SearchInsertPosition {
    /*
    ソート済の整数からなる配列nums,整数targetが与えられる。
    numsの中からtargetを探して見つかればそのインデックスを返す。
    見つからなければ、ソート状態を維持したまま挿入可能な配列のインデックスを返す。
    時間計算量O(log n)のアルゴリズムで実装する必要がある。
    nums = [1, 3, 5, 6], target = 2, output = 1
    nums = [1, 3, 5, 6], target = 7, output = 4
    nums = [1, 3, 5, 6], target = 5, output = 2

    memo:
    二分探索アルゴリズムを実装する問題。
    Rustのslice::partition_pointでそのまま実装できるものの、アルゴリズム自体の実装が求められていると考えるのが自然なので実装する。

    二分探索アルゴリズム実装する時に気をつけることを思い出す。
    - ループ毎に区間が必ず狭まること
    - 区間の開始と終了が閉じているのか、開いているのか
    - targetに対して候補がどちら側の区間に含まれるのか

    Accept
    理解した状態で実装できたものの、採点システムを通す前の見直しに1時間程度かけながら修正したりしていたので、NGとしておく。

    */
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let middle = left + (right - left) / 2;
            let middle_value = nums[middle];

            if middle_value < target {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as i32
    }
}
