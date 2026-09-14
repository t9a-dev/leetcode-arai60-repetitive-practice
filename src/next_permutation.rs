struct NextPermutation {}
impl NextPermutation {
    /*
    問題の理解:
    整数からなる配列numsが与えられる。
    numsに破壊的な変更を加えて次の順列の状態にする。
    次の順列の例:
    [1,2,3] -> [1,3,2] -> [2,1,3] -> [2,3,1] -> [3,1,2] -> [3,2,1]
    [3,2,1]のとき次の順列は[1,2,3]になる。
    降順に並び替えるときの次の状態の順列が答えとなる。

    memo:
    部分問題としては2つの値を比較して昇順であれば入れ替えて返す。
    降順の時の操作がよく分からない。
    手が止まったので答えを見る。

    解法の理解:
    配列の末尾から先頭に向けて2つの値(window)を確認して、昇順になっているwindowの位置を確認する。
    ここで、昇順になっているwindowがなければ全体が降順ソートになっているので、全体を昇順ソートして終了。
    windowの位置+1..の範囲をreverseしているが、この部分は数学的なパズルという感じがする。
    踏み込んで理解するべきかをLLMに相談してみる。
    数学的な証明と言う感じではなさそう。
    - ある時点の配列の内容から降順の並びに近づけたい
      - 配列を数値としてみると、桁を入れ替えつつ少しずつ大きくしていきたいという気持ち
    - 一番小さい桁同士を比較して昇順であれば、入れ替えるだけで答えが得られる
    - そうでなければ次に小さい数字を探して桁を入れ替えるような操作を行っている
    - 入れ替えた桁以降(nums[rfind_first_decreasing_window_index+1..])は昇順つまり最小であってほしいので最後にsortする
      - ここで不変条件によってsortではなくreverseで代替できるので、最適化としてsort -> reverse にすることによって時間計算量をO(n log n) -> O(n)にしている
      - rfind_first_decreasing_window_indexを取得した時点で、右側は降順になっているという不変条件から昇順にするためにsortする必要がなく、reverseするだけでよい

    */
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let Some(rfind_first_decreasing_window_index) = nums.windows(2).rposition(|w| w[0] < w[1])
        else {
            // numsは降順ソートされているので逆順（昇順）にして早期リターン
            nums.reverse();
            return;
        };
        let Some(swap_index) = nums
            .iter()
            .rposition(|v| nums[rfind_first_decreasing_window_index] < *v)
        else {
            unreachable!()
        };
        nums.swap(rfind_first_decreasing_window_index, swap_index);
        nums[rfind_first_decreasing_window_index + 1..].reverse();
    }

    pub fn next_permutation2(nums: &mut Vec<i32>) {
        for pivot_index in (0..nums.len()).rev() {
            for swap_index in (pivot_index..nums.len()).rev() {
                if nums[pivot_index] < nums[swap_index] {
                    nums.swap(pivot_index, swap_index);
                    nums[pivot_index + 1..].reverse();
                    return;
                }
            }
        }
        nums.reverse();
    }
}
