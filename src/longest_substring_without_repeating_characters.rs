use std::collections::{HashMap, HashSet};

struct LongestSubstringWithoutRepeatingCharacters {}
impl LongestSubstringWithoutRepeatingCharacters {
    /*
    問題の理解:
    文字列sが与えられる。重複した文字を含まない部分文字列の最長長さを返す。
    部分文字列とは文字列内の連続した空でない文字の集合のこと。

    memo:
    文字列先頭からstart,endの2ポインタ方式で文字列を走査する。
    endのポインタはループごとに一文字ずつ進める。
    s[start], s[end]のポインタの文字列をHashSetに入れていく。
    重複検知したら、HashSetをクリアする。startを+1, end = start + 1;
    end - start + 1で文字数をカウントして、最長文字数を更新していく。
    文字列数nとしたときの時間計算量がO(n ^ 2)となり、制約から 10 ^ 10 / 10 ^ 8 = 100秒位かかるのでTime LImit Exceededとなると思う。
    他に解法が思いつかないので、採点システム通して、答えを見る。

    数回Wrong Answerとなり、修正を行ったところAcceptedとなった。見積もりとは違ってTime Limit Exceededにはならなかった。
    ロジック的には合っているが、明らかにより効率の良い実装方法がある。
    時間計算量の見積もりをLLMに聞いてみる。
    時間計算量の見積もり自体は間違っていなかった(O(n ^ 2))が、問題の制約からアルファベット、記号、数字、スペースといった合計で100種類未満の文字しか使われない。
    よって、文字種類数C = 100の定数としてO(nC)となり、100 * 10 ^ 5 / 10 ^ 8 = 0.1秒 = 100msくらいになり、LeetCode採点システムの実行時間の桁感はあっている。

    */
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        if s.len() == 1 {
            return 1;
        }

        let chars: Vec<_> = s.chars().collect();
        let mut longest_substring_length = 1;
        let mut start = 0;
        let mut end = 1;
        let mut visited_characters = HashSet::new();
        visited_characters.insert(chars[start]);

        while end < chars.len() {
            if !visited_characters.insert(chars[end]) {
                start += 1;
                end = start + 1;
                visited_characters.clear();
                visited_characters.insert(chars[start]);
                continue;
            }

            longest_substring_length = longest_substring_length.max(end - start + 1);
            end += 1;
        }

        longest_substring_length as i32
    }

    /*
    より効率的な実装
    文字列を先頭から走査していく。
    start = 0として、文字の重複がなければ、startは動かず、endが最後の文字を指した時=文字列全体の長さとなる。
    endが今まで見た文字と同じ文字を見つけた時（重複）startの位置をその文字を除いた位置まで進める。
    このとき、startをより大きいインデックス（文字列終端寄り）に更新していく必要がある。
    startをより小さいインデックス（文字列開始寄り）に更新してしまうと、それまでに見つけた重複文字を部分文字列に含んでしまう可能性があり、正しい答えを得られないため。
    */
    pub fn length_of_longest_substring_2(s: String) -> i32 {
        let mut longest_substring_length = 0;
        let mut character_to_index = HashMap::new();
        let mut start = 0;
        for (end, c) in s.chars().enumerate() {
            if let Some(repeated_character_index) = character_to_index.insert(c, end) {
                start = start.max(repeated_character_index + 1);
            }
            longest_substring_length = longest_substring_length.max(end - start + 1);
        }

        longest_substring_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::LongestSubstringWithoutRepeatingCharacters;

    #[test]
    fn length_of_longest_substring_test() {
        assert_eq!(
            LongestSubstringWithoutRepeatingCharacters::length_of_longest_substring(
                "abcabcbb".to_string()
            ),
            3
        );
        assert_eq!(
            LongestSubstringWithoutRepeatingCharacters::length_of_longest_substring_2(
                "abcabcbb".to_string()
            ),
            3
        );
    }
}
