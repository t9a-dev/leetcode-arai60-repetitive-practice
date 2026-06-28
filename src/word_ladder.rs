use std::collections::{HashMap, HashSet, VecDeque};

struct WordLadder {}

impl WordLadder {
    /*
    問題の理解:
    文字列begin_wordとend_word、文字列からなる配列word_listが与えられる。
    文字列のペアのうち、一文字違いの文字列を隣接文字列として扱う。
    begin_wordからend_wordまでの最短変換シーケンスに含まれる単語の数を返す。シーケンスが存在しないときは0を返す。
    begin_wordからword[i]の隣接文字列を辿っていき、end_wordになる最短の文字列数を知りたい。
    制約
    - begin_wordは必ずしもword_listに含まれている必要はない。
    - word_listの末尾にend_wordが含まれる
    - wordは英字小文字からなる

    memo:
    隣接文字列（1文字違いの文字列）かを判定する関数が必要
    HashMapで文字列同士の隣接リストを作る
    再帰処理で隣接リストを辿りながら、end_wordを見つけるまでに辿った単語の数を数え上げる。
    一度の探索で同じワードを複数回見ないように、HashSetでチェック済文字列を管理する。
    複数の経路があるので、それぞれの経路の内最短のものを返す。

    手が止まったので答えを写経する

    解法の理解:
    考え方としてはほぼ合っているが、コードにするまで距離があり手が止まった感じ。
    再帰処理によるDFS（深さ優先探索）ではなく、BFS（幅優先探索）の方が良い。根（begin_word）からの最短距離を探すので、根から近い順に探していって見つけたら探索打ち切りするイメージ。
    再帰処理でも、できなくはないが各経路の距離を別途保存しておいて、最後に最短経路を探す手順が必要になる。
    文字列同士の隣接判定実装は、どれだけ同じ文字があるかより、異なる文字を見つけたというカウントで早期リターンしているのは思いつかなかった。

    より速い実装
    https://leetcode.com/problems/word-ladder/solutions/1765599/rust-4-implementations-150ms-150ms-30ms1-s55f
    最初にまとめて隣接リストを作らないことで高速化している。

    */
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_to_adjacency_words: HashMap<&str, Vec<&str>> = HashMap::new();
        let merged_word_list = vec![begin_word.clone()]
            .into_iter()
            .chain(word_list)
            .collect::<Vec<String>>();
        for (i, word1) in merged_word_list.iter().enumerate() {
            for word2 in merged_word_list.iter().skip(i + 1) {
                if !Self::is_adjacency_word(word1, word2) {
                    continue;
                }
                word_to_adjacency_words
                    .entry(word1)
                    .or_default()
                    .push(word2);
                word_to_adjacency_words
                    .entry(word2)
                    .or_default()
                    .push(word1);
            }
        }

        let mut visited_words: HashSet<_> = HashSet::new();
        let mut frontier = VecDeque::from_iter([(begin_word.as_str(), 1)]);
        while let Some((word, ladder_count)) = frontier.pop_front() {
            if word == end_word {
                return ladder_count;
            }

            let Some(adjacency_words) = word_to_adjacency_words.get(word) else {
                continue;
            };
            for adjacency_word in adjacency_words {
                if !visited_words.insert(adjacency_word) {
                    continue;
                }
                frontier.push_back((adjacency_word, ladder_count + 1));
            }
        }

        0
    }

    fn is_adjacency_word(a: &str, b: &str) -> bool {
        let mut diff_count = 0;
        for (ac, bc) in a.chars().zip(b.chars()) {
            if ac != bc {
                diff_count += 1;
            }

            if 1 < diff_count {
                return false;
            }
        }

        diff_count == 1
    }
}
