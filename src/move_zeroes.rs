struct MoveZeroes {}
impl MoveZeroes {
    /*
    問題の理解:
    整数からなる配列numsが与えられる。
    numsの要素の内、ゼロでない要素の相対的な位置を保ちながら全ての0を配列末尾に移動する。
    配列のコピーを作成せず、元の配列を直接変更する方法で行う必要がある。

    memo:
    ソートはできない。相対的な位置を壊してしまうため。
    numsをインプレイスで並べ替える実装が思いつかない。
    手が止まったので答えを見る。

    所感:
    実装を見てすぐに理解できるので解けなかったのが歯がゆい感じ。
    */
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut swap_target_index = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }
            nums.swap(i, swap_target_index);
            swap_target_index += 1;
        }
    }

    /*
    Erase-remove idiom と呼ばれる実装。
    この実装を思い出したかったが、思い出せなかった。
    インプレイスで条件に一致する値のみにしたいという場面でretainメソッドを使えるようになりたいなという感じ。
    */
    pub fn move_zeroes_2(nums: &mut Vec<i32>) {
        let original_size = nums.len();
        nums.retain(|&num| num != 0);
        nums.resize(original_size, 0);
    }
}
