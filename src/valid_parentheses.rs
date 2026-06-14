use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Deref,
};

struct ValidParentheses {}
impl ValidParentheses {
    /*
    '(',')','{','}','[',']'からなる文字列sが与えられる
    文字列に含まれる文字が全て有効なペアであればtrueを返す
    そうでなければfalseを返す
    開き記号 '(', '[', '{' を見つけたら open_brracketsにpushする
    閉じ記号 ')', ']', '}' を見つけたら、open_brracketsからpopして正しいペアなのかを判定。
    正しくなければ早期リターンでfalse
    open_bracketsが空であればtrue

    自力で解けた。

    */
    pub fn is_valid(s: String) -> bool {
        let mut open_brackets = Vec::new();
        let close_to_open_brackets: HashMap<_, _> =
            HashMap::from_iter([(')', '('), (']', '['), ('}', '{')]);

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => open_brackets.push(c),
                _ => {
                    let Some(actual_open_bracket) = open_brackets.pop() else {
                        return false;
                    };
                    let Some(expect_open_bracket) = close_to_open_brackets.get(&c) else {
                        return false;
                    };
                    if actual_open_bracket != *expect_open_bracket {
                        return false;
                    }
                }
            }
        }

        open_brackets.is_empty()
    }

    // 写経
    pub fn is_valid2(s: String) -> bool {
        let mut close_brackets = Vec::new();

        for c in s.chars() {
            match c {
                '(' => close_brackets.push(')'),
                '[' => close_brackets.push(']'),
                '{' => close_brackets.push('}'),
                ')' | ']' | '}' if Some(c) != close_brackets.pop() => return false,
                _ => (),
            }
        }

        close_brackets.is_empty()
    }

    /*
    記号のハードコーディングを減らしたバージョン
    3種類の記号しか扱わないならHashMapはオーバーヘッドが大きいのでハードコーディングで良いかなという感じ
    扱う記号の種類が多いときはこっちのほうが良いと思った
    */
    pub fn is_valid3(s: String) -> bool {
        let mut frontier = Vec::new();
        let close_to_open_brackets: HashMap<_, _> =
            HashMap::from_iter([(')', '('), (']', '['), ('}', '{')]);
        let open_brackets = close_to_open_brackets.values().collect::<HashSet<_>>();

        for c in s.chars() {
            match c {
                _ if open_brackets.contains(&c) => frontier.push(c),
                _ if frontier.pop() != close_to_open_brackets.get(&c).copied() => return false,
                _ => (),
            }
        }

        frontier.is_empty()
    }

    /*
    LLMに聞いて、よりデータ駆動にリファクタリング
    */
    pub fn is_valid_data_driven(s: String) -> bool {
        let pairs = [('(', ')'), ('[', ']'), ('{', '}')];
        let open_brackets = pairs.iter().map(|(open, _)| open).collect::<HashSet<_>>();
        let close_to_open = pairs
            .into_iter()
            .map(|(open, close)| (close, open))
            .collect::<HashMap<_, _>>();

        let mut frontier = Vec::new();
        for c in s.chars() {
            match c {
                _ if open_brackets.contains(&c) => frontier.push(c),
                _ if frontier.pop() != close_to_open.get(&c).copied() => return false,
                _ => (),
            }
        }

        frontier.is_empty()
    }
}
