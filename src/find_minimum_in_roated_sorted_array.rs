struct FindMinimumInRoatedSortedArray {}
impl FindMinimumInRoatedSortedArray {
    /*
    問題の理解:
    長さnの昇順にソートされた整数からなる配列numsが与えられる。配列に含まれる値のうち最小値を返す。
    配列に含まれる整数は一意である。
    時間計算量O(log n)のアルゴリズムで実装する必要がある。
    nums = [0, 1, 2, 3]のとき、2回転していると、nums = [2, 3, 0, 1]

    memo:
    全体がソート済であるか、ソート済である区間が2つに分かれているケースが考えられる。
    時間計算量O(log n)のアルゴリズムで実装する必要があるので、二分探索を利用する。
    left と middle, middle と right, さらにleftとrightの値の大小関係を見ることで最小値がleft, rightのどちらの区間にあるかが判断できる。
    この判定を二分探索の中で行うことで最小値が見つけられると思う。
    candidate_in_left, candidate_in_rightのような感じで左右どちらの区間に候補があるかを考える。
    candidate_in_right = nums[right] < nums[left] and nums[right] < nums[middle] then left = middle + 1
    else right = middle

    nums = [4, 5, 1, 2, 3]

    Wrong Answerとなった。nums = [2, 1]

    答えを見る。

    解法の理解:
    もっとシンプルにより小さい値がpivot(nums[middle])から見て左側にありそうか、右側にありそうかと考えるべきだった。
      - nums[middle] < nums[right]が成り立つ時点で、右側の値（より大きい値の区間）は捨てることができる。
      - nums[middle] < nums[right]が成り立たないとき、回転しているのでnums[middle]を含む左側の値は捨てることができる。
    nums[middle] < nums[right] then right = middle
    else left = middle + 1
    */
    pub fn find_min_wrong_answer(nums: Vec<i32>) -> i32 {
        /*
        Wrong Answer
        */
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let middle = left + (right - left) / 2;
            let is_candidate_in_right =
                nums[right - 1] < nums[left] && nums[right - 1] < nums[middle];

            if is_candidate_in_right {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        nums[left]
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let middle = left + (right - left) / 2;
            if nums[middle] < nums[right] {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        nums[left]
    }
}
