use std::collections::HashMap;

struct WordBreak {}
impl WordBreak {
    /*
    問題の理解:
    文字列sと単語辞書word_dictが与えられる。
    sがスペースで区切られた1つ以上の辞書単語の連続した文字列に分割できるかどうかを判定し、可能であればtrueを返す。
    辞書内の単語は分割処理で複数回利用されることがある。

    memo:
    word_dict[i]を選んだ時の残りの文字列を状態として持つ。
    最終的にsの文字列が空に慣れば良い。
    ある単語を選んだら、sからすべて取り除きたい。
    文字を取り回すとコストが掛かりそうなので文字数などで代用したほうが良さそう。
    手が止まったので答えを見る。

    解法の理解:
    https://www.youtube.com/watch?v=Sx9NNgInc3A
    word_dict[i]と同じ文字数分をsの部分文字列と比較する。
    dp[s.len()] = trueとして、dp[0]の状態が解となるような状態遷移を作る。
    dp[i] = s[i..word_dict[j].len()] == word_dict[j]
    気持ちとしては、
    - word_dictのwordで文字列sがすべて分割可能かを知りたい
    - word_dict[j]の文字列長と同じ長さの文字列を文字列sから確認する
      - i = 0 から初めて、word_dict[j].len() == s[i..word_dict[j].len()]を見つけるたびに、i += word_dict[j].len()とする
      - word_dictのwordで文字列sがすべて分割可能だとi == s.len()となる

    i = 0から初めて、word_dict[j]と一致するたびにiを進めていき、i == s.len()となればtrueと判定できる。
    つまり、dp[s.len()] = trueとして、dp[0] = dp[i + word_dict[j].len()]で求められる。iはs.len()-1から0に向けて単調減少する。
    */
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut breakable_word = vec![false; s.len() + 1];
        breakable_word[s.len()] = true;

        for i in (0..s.len()).rev() {
            for word in &word_dict {
                let Some(s_word) = s.get(i..i + word.len()) else {
                    continue;
                };
                if s_word != word {
                    continue;
                }

                breakable_word[i] = breakable_word[i + word.len()];
                if breakable_word[i] {
                    break;
                }
            }
        }

        breakable_word[0]
    }

    pub fn word_break2(s: String, word_dict: Vec<String>) -> bool {
        let mut index_to_breakable = HashMap::new();
        Self::is_breakable_word(&s, &word_dict, 0, &mut index_to_breakable)
    }

    fn is_breakable_word(
        s: &str,
        word_dict: &[String],
        i: usize,
        index_to_breakable: &mut HashMap<usize, bool>,
    ) -> bool {
        if s.len() == i {
            return true;
        }
        if let Some(is_breakable) = index_to_breakable.get(&i) {
            return *is_breakable;
        };

        let breakable = word_dict.iter().any(|word| {
            let Some(s_word) = s.get(i..i + word.len()) else {
                return false;
            };
            if s_word != word {
                return false;
            }
            Self::is_breakable_word(s, word_dict, i + word.len(), index_to_breakable)
        });
        index_to_breakable.insert(i, breakable);

        breakable
    }
}
