struct MinimumSizeSubArraySum {}
impl MinimumSizeSubArraySum {
    /*
    問題の理解:
    正の整数からなる配列numsと正の整数targetが与えられる。
    numsの部分配列の和がtarget以上となる最小の長さを返す。和がtargetとなるような部分配列がなければ0を返す。

    memo:
    配列の中から一番大きい値を見つける。
    一番大きい値のインデックスを起点として、left, right方向にインデックスを伸ばしつつ和がtarget以上になるかを見る。
    left, rightの位置の値のうち大きい値の方に伸ばしていく
    n = nums.len() として時間計算量はO(n)になると思う。最初に一度一番大きい値を探すして、そのあと、最大でn回ループする。
    分割統治法で行けそうな気がしてきた。
    手が止まったので答えをみる。

    解法の理解:
    start, endnの2ポインタで部分配列の範囲を管理する。
    nums先頭から走査する。 start, end = 0, 0
    部分配列の和がtarget以上であれば、start += 1として範囲を縮めながらより短い部分配列を探す。
    部分配列の和がtarget未満であれば、end += 1として範囲を広げながら部分配列の和が大きくなる方向で探す。
    部分配列の和は都度計算するのではなく、ポインタ更新時に計算する。
    */

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        const NOT_FOUND: i32 = 0;
        if nums.is_empty() {
            return NOT_FOUND;
        }
        let sentinel_sub_array_length = nums.len() as i32 + 1;
        let mut min_sub_array_length = sentinel_sub_array_length;
        let mut sub_array_sum = nums[0];
        let (mut start, mut end) = (0, 0);
        while start <= end && end < nums.len() {
            if target <= sub_array_sum {
                min_sub_array_length = min_sub_array_length.min((end - start) as i32 + 1);
                sub_array_sum -= nums[start];
                start += 1;
                continue;
            }

            end += 1;
            if let Some(&v) = nums.get(end) {
                sub_array_sum += v;
            }
        }

        if sentinel_sub_array_length == min_sub_array_length {
            return NOT_FOUND;
        }
        min_sub_array_length
    }

    /*
    n = nums.len()
    累積和の配列を二分探索することで時間計算量がO(n log n)となる実装
    nums[i]は正の整数なので累積和が必ず増加する、つまりprefix_sumsはソートされた状態になるので二分探索可能になっている。
    数学パズルに感じる。
    */
    pub fn min_sub_array_len_2(target: i32, nums: Vec<i32>) -> i32 {
        const NOT_FOUND: i32 = 0;
        if nums.is_empty() {
            return NOT_FOUND;
        }

        let mut nums_iter = nums.iter();
        let prefix_sums = std::iter::successors(Some(0), |x| nums_iter.next().map(|num| num + x))
            .collect::<Vec<_>>();
        let sentinel_sub_array_length = nums.len() + 1;
        let mut min_sub_array_length = sentinel_sub_array_length;
        for start in 0..nums.len() {
            let end =
                prefix_sums.partition_point(|&prefix_sum| prefix_sum < target + prefix_sums[start]);

            if end == prefix_sums.len() {
                break;
            }

            min_sub_array_length = min_sub_array_length.min(end - start);
        }

        if sentinel_sub_array_length == min_sub_array_length {
            return NOT_FOUND;
        }
        min_sub_array_length as i32
    }
}
