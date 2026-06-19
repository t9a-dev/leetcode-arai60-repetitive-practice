use std::collections::HashMap;

struct GroupAnagrams {}
impl GroupAnagrams {
    /*
    問題の理解:
    文字列を要素として持つ配列が与えられる。
    それぞれの文字列の要素の内、同じ文字から構成される文字列を配列にグループ化して、これらの配列を配列に入れて返す。
    ["bat",nat","tea"] -> [["bat"],["nat","tea"]]のように並び順が違うだけで、文字列を構成する文字自体は同じものをグループ化する。

    memo:
    文字列を一ずつ走査しながら、ソートした結果をHashMapのキーとして登録していく。文字列自体は元の値を、キーに対応する値として追加する。
    このときすでに登録済のキーであれば、キーに対応する値として文字列を追加する。
    この方法だと全ての文字列について、毎回ソートしながらアナグラムかどうかを確認する必要がある。
    入力の制約としては、
        - 1 <= strs.length <= 10 ^ 4
        - 0 <= strs[i].length <= 100
    時間計算量 O(n * k log k)
        - 文字列を毎回ソートするので、文字数kとしたときO(k log k)
        - 文字列の数をnとするとO(n) 
    入力の上限値を使って時間計算量を見積もる。
    O( 10 ^ 4 * 100 log 100) = 2,000,000 となり問題なさそう。
        - 2,000,000 / 10 ^ 8 ≠ 0.02s = 20ms でTime Limit Exceededを気にするようなレベルではなさそう（悲観的な見積もりなので）
            - LeetCodeの実行時間は当てにならないが6msだった
    補助空間計算量について
    文字列の個数n, 文字列毎の文字数mとしてO(n * m)
        - Rustのcharは4 bytes
        - 4bytes文字列（ascii lower case）なので、文字列あたり400byte
        - この文字列が10 ^ 4個あるので、400 * 10 ^ 4 = 4,000,000 = 約4MB
    HashMapで管理する都合上全ての文字列を新たに乗せても約4MBなので問題なさそう。
        - LeetCodeの実行環境は当てにならないが5.07MBだった

    Accepted
    LeetCodeのBeatsが35%あたりなのでもう少し速い実装がありそう。
    chars.sort()で並び替えている部分を別の方法で実装している。
    問題の入力制約からascii_lowercase('a'~`z`)しか来ないので、文字列を長さ26の配列で表して、どの文字が含まれているかを配列で表現。
    "abcz" = [1,1,1,0,0....,1] となるようなイメージ。
    文字列を構成する文字を配列で表現することでソート部分の時間計算量を削減している。
    */
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_to_group = HashMap::new();
        for word in strs {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();

            anagram_to_group
                .entry(chars)
                .and_modify(|group: &mut Vec<String>| {
                    group.push(word.clone());
                })
                .or_insert(vec![word]);
        }

        let mut anagram_groups = Vec::new();
        for group in anagram_to_group.values() {
            anagram_groups.push(group.clone());
        }

        anagram_groups
    }

    /*
    リファクタリング
    余分なcloneの除去
    */
    pub fn group_anagrams2(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_to_group: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for word in strs {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();

            anagram_to_group.entry(chars).or_default().push(word);
        }

        anagram_to_group.into_values().collect()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn playground() {
        // 'a' ascii code 97
        // https://www.ascii-code.com/97
        let offset = 'a' as usize;
        assert_eq!(offset, 97);

        // 'a'を基準(0)とした時に'z'は25
        let z_ascii_code = 'z' as usize;
        assert_eq!(z_ascii_code, 122);
        assert_eq!(z_ascii_code - offset, 25);
    }
}
