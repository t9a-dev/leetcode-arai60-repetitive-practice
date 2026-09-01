use std::collections::{HashMap, VecDeque};

struct IsSubsequence {}
impl IsSubsequence {
    /*
    問題の理解:
    2つの文字列sとtが与えられたとき、sがtの部分列であればtrueを返し、そうでなければfalseを返す。
    文字列の相対的な並び順は変えずに判定する必要がある。
    "ace" は "abcde" の部分列だが、"aec"は部分列ではない。

    memo:
    文字列の相対的な順序を保つ必要があるので、tの文字とindexをもつHashMapを作って最後に見つけた文字の位置を確認しながら部分列があるか見れば良さそう。
    tに同じ文字が複数回あるときのことを考慮して文字をkey, indexのリストを昇順で値として持つ。
    部分文字列の判定として利用したindexはpopして取り除く。
    最後に見つけた文字のindexを記録しておいて、見つけた文字のindex常に大きくなることを確認する。
    Wrong Answerとなった。
    - sが空の時常にtrueとなる
    - tの後ろ側に部分列がある場合に、部分列ではないならびの文字列が前半部分に存在するケースに対応できない
      - s="ab", t="baab"
    答えを見る

    解法の理解:
    sのindex,tのindexの2ポインタを使って0から初めて、最終的にsのポインタがs.len()と等しくなれば良い。

    */
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let mut s_index = 0;
        let mut t_index = 0;

        while s_index < s.len() && t_index < t.len() {
            if s[s_index] == t[t_index] {
                s_index += 1;
            }
            t_index += 1
        }

        s_index == s.len()
    }

    // 二分探索を利用した実装
    pub fn is_subsequence_2(s: String, t: String) -> bool {
        let mut t_character_to_indexes: HashMap<_, Vec<_>> = HashMap::new();
        for (i, c) in t.chars().enumerate() {
            t_character_to_indexes.entry(c).or_default().push(i);
        }

        let mut previous_t_index = 0usize;
        for c in s.chars() {
            let Some(t_character_indexes) = t_character_to_indexes.get(&c) else {
                return false;
            };
            // partition_pointは引数の条件でfalse,trueになる2つの配列に分けたときに、2つ目の配列の先頭のインデックスを返す。
            let t_character_index = t_character_indexes.partition_point(|&i| i < previous_t_index);
            if t_character_index == t_character_indexes.len() {
                return false;
            }
            previous_t_index = t_character_indexes[t_character_index] + 1;
        }

        true
    }
}
