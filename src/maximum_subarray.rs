use core::num;

struct MaximumSubarray {}
impl MaximumSubarray {
    /*
    問題の理解:
    整数からなる配列numsが与えられる。numsの部分列おいて、最大の合計値となる部分列を探して、その合計値を返す。

    memo:
    最小問題としてnums[i]が初期状態で最大の合計値だと考えられる。
    マイナス値が含まれる点が注目すべき点だと思う。加算した時に欲しい答えから離れてしまうので。
    分割統治法、再帰処理でnums.len() < 2をベースケースとして、加算した時、そのまま返したときに一番大きい値を返せば良さそう。

    WrongAnswerとなった。部分配列は連続している必要があり、途中の値を飛ばして虫食いにすることができない。

    手が止まったので答えを見る

    解答の理解:
    累積和の考え方で先頭から累積和の方が大きいのか、累積和よりも値単体のほうが大きいのかを見ている。
    この手順の中で一番大きい累積をを保持しておき、最大が見つかるたびに更新する。
    最終的に最大の累積和を返す。部分列自体を構成する値の集合は情報として必要ないので。

    */
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut max_sum = nums[0];
        let mut sum = nums[0];
        for num in nums.into_iter().skip(1) {
            sum = num.max(num + sum);
            max_sum = max_sum.max(sum);
        }

        max_sum
    }

    /*
    分割統治法の解法
    - 再帰で左右で分割しながら、最大の累積和を返す。
    - ベースケースとして値が1つの時はそれ自体が累積和と考えられる。
    - マイナス値があるので、常に加算した方が累積和が大きくなるとは限らない。
    - 左右で分割するので、連続する配列をみるように気をつける必要がある。
    - 左右で分割する時のleft ~ middle ~ rightとなるので、left,rightをまたぐ配列を見られるように、middle -> left, middle -> rightの方向に走査する必要がある。
        - ここが難しく感じる。左右に分割した時に、middle -> left ,middle -> right の方向で操作すれば左右をまたぐ部分列も正しく扱えるという点が直感的に分かりづらく感じる。
    - 常にmiddleの値を含むという不変条件を維持することで左右をまたいだ連続する部分列も扱えている。
    - なるべく考えることが少なくなる小さな部分問題にして、その部分問題をより大きなパターンに当てはめてうまく動くか考える感じだろうか。
    */

    pub fn max_sub_array_divide(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        Self::explore_max_sub_array(&nums, 0, nums.len())
    }

    fn explore_max_sub_array(nums: &[i32], left: usize, right: usize) -> i32 {
        if left == right - 1 {
            return nums[left];
        }

        let middle = left + ((right - left) / 2);
        let left_max = Self::explore_max_sub_array(nums, left, middle);
        let right_max = Self::explore_max_sub_array(nums, middle, right);

        let mut sum = 0;
        let mut max_in_left = i32::MIN;
        for i in (left..middle).rev() {
            sum += nums[i];
            max_in_left = max_in_left.max(sum);
        }

        sum = 0;
        let mut max_in_right = i32::MIN;
        for i in middle..right {
            sum += nums[i];
            max_in_right = max_in_right.max(sum);
        }

        let max_in_merged = max_in_left + max_in_right;
        max_in_merged.max(left_max).max(right_max)
    }
}
