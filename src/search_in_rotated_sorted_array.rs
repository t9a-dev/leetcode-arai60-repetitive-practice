struct SearchInRotatedSortedArray {}
impl SearchInRotatedSortedArray {
    /*
    問題の理解:
    整数からなる昇順ソート済の配列numsと整数targetが与えられる。
    numsはソート済の状態から任意の位置で回転している可能性がある。
    targetがnumsに含まれている時、targetのインデックスを返す。見つからないときは -1を返す。
    時間計算量O(log n)のアルゴリズムで実装する必要がある。

    memo:
    nums = [4, 0, 1, 2, 3] target = 0 out = 1
    nums = [4, 0, 1, 2, 3] target = 4 out = 0
    nums = [4, 0, 1, 2, 3] target = 1 out = 2
    nums = [4, 0, 1, 2, 3] target = 3 out = 4

    nums = [0, 1, 2] target = 2 out = 2
    nums = [0, 1, 2] target = 1 out = 1
    nums = [0, 1, 2] target = 0 out = 0

    時間計算量の制約から、二分探索のアルゴリズムで実装する。
    回転しているので、pivot（middle）から見てtargetの値がどちら側にありそうかという考え方で実装できそう。
    配列全体が回転しているかというよりは、回転していない区間にtargetが含まれるかを確認する。
    回転している区間の範囲ではtargetが含まれるかどうかを判定できないため。
    nums[middle] < nums[right] が回転していれば、もう一方の区間 nums[left] < nums[middle] は回転していない。
    昇順ソート済の配列が回転しているという性質から成り立つ。
    回転していない区間にtargetが含まれていなければ、もう一方の区間に含まれているかもしれないという考え方で二分探索する。

    2回 Wrong Answerとなった。
    1度目のWrong Answer:
    while loopの中でtargetと等しいかを判定しており、nums.len() == 1 のときにloopに入らず常に-1を返していた箇所でWrong Answer

    2度目のWrong Answer:
    nums[left] <= target とするべきところを nums[left] < target としていた。
    つまり、左側に含めるべきtargetを飛ばしてしまう条件になっていた。
    */
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let middle = left + (right - left) / 2;

            let rotated_middle_to_right = nums[right] < nums[middle];
            if rotated_middle_to_right {
                if nums[left] <= target && target <= nums[middle] {
                    right = middle;
                } else {
                    left = middle + 1;
                }
            } else {
                if nums[middle] < target && target <= nums[right] {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            }
        }

        if target == nums[left] {
            return left as i32;
        }
        -1
    }
}
