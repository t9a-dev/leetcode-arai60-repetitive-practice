use std::collections::HashMap;

struct FirstUniqueCharacterInAString {}

impl FirstUniqueCharacterInAString {
    /*
    文字列sが与えられる。
    文字列の中で一意になる最初の文字のindexを返す。
    一意になる文字が見つからない時は-1を返す。

    memo:
    文字列の文字をキー、出現頻度を値としてHashMapを作る。
    文字列を最初から走査しながら、出現頻度を確認して1を見つけたら早期リターンでインデックスを返す。
    関数最後まで到達したら-1を返す。

    Accepted
    */
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_to_count: HashMap<_, usize> = HashMap::new();
        for c in s.chars() {
            char_to_count
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for (i, c) in s.chars().enumerate() {
            if char_to_count.get(&c).is_some_and(|count| *count == 1) {
                return i as i32;
            }
        }

        -1
    }
}
