use std::collections::{HashMap, HashSet, VecDeque};

struct WordLadder {}
impl WordLadder {
    /*
     * 問題の理解:
     * 文字列からなる配列word_list,begin_word,end_wordが与えられる。
     * word_listの長さをkとしたとき、end_word == word_list[k]となる。
     * begin_wordから初めて一文字違いのword_list[i]を辿りながらend_wordにたどり着くまでの最小単語数を返す。
     * このような経路がない場合は0を返す。
     * begin_wordはword_listに含まれる必要はない。
     *
     * memo:
     * 隣接リストを作って、隣接する単語を辿りながら数を数える感じだと思う。
     * 隣接する単語をどのように判定するか。 -> 比較したい単語a,bを引数にとって、異なる単語数が1を超えたら早期リターンする関数を作る。
     * end_word と等しい単語を見つけたらそれまでに辿った単語数を返す。
     * Wrong Answerとなった。時間かけすぎたので答えを見る。
     *
     * 解法の理解:
     * 全てのパスの最短経路が欲しいことを考えると、DFSで実装するのではなく、BFSで浅い位置から全ての層を確認する方が良い。
     * こちらのほうが最短経路を更新していく必要がなくなる。
     * 一度辿った経路をメモ化しないと、隣接リストを辿るループで無限ループしうる。
     * 隣接の判定は異なる文字の差が1文字ぴったりであるかで判定するべき。
     * 完全に同一の文字列は隣接ワードではないため。
     *
     */
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_to_adjacency_words: HashMap<_, Vec<_>> = HashMap::new();
        let mut merged_word_list = vec![&begin_word];
        merged_word_list.extend(&word_list);
        for (i, word1) in merged_word_list.iter().enumerate() {
            for word2 in merged_word_list.iter().skip(i + 1) {
                if !Self::is_adjacency_word(word1, word2) {
                    continue;
                }

                word_to_adjacency_words
                    .entry(word1)
                    .and_modify(|words| words.push(word2))
                    .or_insert(vec![word2]);
                word_to_adjacency_words
                    .entry(word2)
                    .and_modify(|words| words.push(word1))
                    .or_insert(vec![word1]);
            }
        }

        let mut frontier = VecDeque::from_iter(vec![(&begin_word, 1)]);
        let mut visited_words: HashSet<_> = HashSet::new();
        while let Some((word, ladder_count)) = frontier.pop_front() {
            if word == &end_word {
                return ladder_count;
            }
            if !visited_words.insert(word) {
                continue;
            }
            let Some(adjacency_words) = word_to_adjacency_words.get(&word) else {
                continue;
            };
            for word in adjacency_words {
                frontier.push_back((word, ladder_count + 1));
            }
        }

        0
    }

    fn is_adjacency_word(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut diff_count = 0;
        for (a, b) in a.chars().zip(b.chars()) {
            if a != b {
                diff_count += 1;
            }
            if 1 < diff_count {
                return false;
            }
        }

        diff_count == 1
    }
}
