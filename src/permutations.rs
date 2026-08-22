use std::collections::VecDeque;

struct Permutations {}
impl Permutations {
    /*
    問題の理解:
    異なる整数からなる配列numsが与えられる。
    整数からなる可能なすべての順列を返す。順列の並びは任意で良い。
    順列とは要素を並べ替えたときに取り得る全ての組み合わせ。

    memo:
    手が止まったので答えを見る。

    解法の理解:
    NeetCodeの解説動画
    https://www.youtube.com/watch?v=s7AvT7cGdSo
    決定木として考える。
    配列から1つの値を取り除いて残った配列の末尾に値を追加して順列を生成することを繰り返す。
    常に1つの要素を選んだ時と、選ばなかった要素の組み合わせを見たいのでnums.pop_frontして取り出したあとに、nums.push_backしている。

    所感:
    base caseでvec![nums]を返しているのは分かるが、resultを再帰の結果として返しているがややこしく感じる。
    LLMによると、recursive caseの中身を頭の中で追うのではなく、どのような戻り値であるかを固定して考える。
    どのような戻り値であるかを決めたらそれを返す中身を考えるという考え方もおすすめとのこと。
    */
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![nums];
        }

        let mut result = Vec::new();
        let mut nums: VecDeque<_> = VecDeque::from_iter(nums);
        for _ in 0..nums.len() {
            let Some(num) = nums.pop_front() else {
                continue;
            };
            let mut permutes = Self::permute(nums.clone().into());
            permutes.iter_mut().for_each(|permute| permute.push(num));
            result.extend(permutes);
            nums.push_back(num);
        }

        result
    }
}
