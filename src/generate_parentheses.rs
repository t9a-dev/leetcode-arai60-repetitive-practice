struct GenerateParentheses {}
impl GenerateParentheses {
    /*
    問題の理解:
    n組の括弧が与えられた時、有効な括弧の組み合わせを全て生成して返す。
    有効な括弧とは`()`のように開き括弧と綴じ括弧が対応していること。`())(`は有効な閉じ括弧ではない。

    memo:
    決定木を書いてみたがよく分からない。
    n=1 `()` が再帰におけるbase caseになるように見える。
    手が止まったので答えを見る。

    解法の理解:
    `(`の数はnと等しくなる。
    ')'の数が'('の数を超えると不正なペアが生まれる。
    '('から必ず始まる。
    '('の数 = ')'の数 = n のときの状態が正しいペアになる。

    所感:
    そもそも解法が思いつかないなと言う感じ。
    */
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut parenthesis = String::new();
        let mut all_parenthesis = Vec::new();
        Self::make_parentheses(n, 0, 0, &mut parenthesis, &mut all_parenthesis);

        all_parenthesis
    }

    fn make_parentheses(
        n: i32,
        open_count: i32,
        close_count: i32,
        parenthesis: &mut String,
        all_parenthesis: &mut Vec<String>,
    ) {
        if open_count == n && close_count == n {
            all_parenthesis.push(parenthesis.clone());
            return;
        }

        if open_count < n {
            parenthesis.push('(');
            Self::make_parentheses(n, open_count + 1, close_count, parenthesis, all_parenthesis);
            parenthesis.pop();
        }

        if close_count < open_count {
            parenthesis.push(')');
            Self::make_parentheses(n, open_count, close_count + 1, parenthesis, all_parenthesis);
            parenthesis.pop();
        }
    }

    pub fn generate_parenthesis2(n: i32) -> Vec<String> {
        let mut all_parenthesis = Vec::new();
        let mut frontier = Vec::new();
        frontier.push((String::from(""), 0, 0));

        while let Some((mut parenthesis, open_count, close_count)) = frontier.pop() {
            if open_count == n && close_count == n {
                all_parenthesis.push(parenthesis);
                continue;
            }
            if open_count < n {
                parenthesis.push('(');
                frontier.push((parenthesis.clone(), open_count + 1, close_count));
                parenthesis.pop();
            }
            if close_count < open_count {
                parenthesis.push(')');
                frontier.push((parenthesis.clone(), open_count, close_count + 1));
                parenthesis.pop();
            }
        }

        all_parenthesis
    }
}
