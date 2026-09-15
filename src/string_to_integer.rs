struct StringToInteger {}
impl StringToInteger {
    /*
    問題の理解:
    文字列sが与えられるので次のルールで符号付き32bit整数に変換して返す。
    - 先頭の空白は無視する
    - -符号がついているときはマイナス値として扱う。+又は符号無しで自然数として扱う。
    - 先頭の0はスキップしながら整数を読み取る。数字以外の文字が現れるか文字列末尾に到達するまで読み進める。
    数字が出現しなければ0として返す。
    - 数値が符号付き32bit整数を溢れる場合は、- 2 ^ 31 ,2 ^ 31 - 1 に丸めて返す。

    memo:
    ナイーブな実装で時間計算量O(n)になりそう。
    問題の制約から文字列の長さは最大200なので、64bit符号付き整数では扱いきれない。
    文字列から数値のみを抽出して、桁溢れするようであれば丸めるという考え方の方が良さそう。
    符号を見つけた時点で、それ以降に文字列を見つけたら探索を打ち切る必要がある。
    数字を見つけた時点で、それ以降に文字列を見つけたら探索を打ち切る必要がある。

    ナイーブにifによる分岐でWrongAnswerとなったので答えを見る。

    解法の理解:
    手続き的に考えている。
    - 先頭の空白を取り除く
    - 符号を確認する
      - 符号であればイテレータを1つ進める。
    - 数字ではない文字を見つけるまでイテレータを進める
    - 数字を集めたときのイテレータが空であれば0を返す。
    - オーバーフローしないか最下位桁を1桁ずつ確認している。
      - num > i32::MAX / 10 : i32の上限値から最下位桁を取り除いたときにnumが超えていないかを見ている。
        numに一桁増やす余地があるかをチェックしている
      - num == i32::MAX / 10 && digit > i32::MAX % 10 : numとi32::MAX / 10 が等しくて、i32::MAXの最下位桁の数値よりも、
        増やそうとしている桁の数値(digit)が大きいとオーバーフローすることをチェックしている。
    i32::MAX = 2 ^ 31 - 1
    i32::MIN = 2 ^ 31
    i32::MAXを反転させてもi32::MINに収まるので、最後に符号を反転させる部分ではチェックしなくても良い。
    機械に任せられる部分（オーバーフローチェック）は機械に任せたいので読み手に不安を与えるよりはchecked_mulを使いたいなという気持ちはある。

    2026/9/15現在、組み込み関数で文字列（複数の文字）から数値に変換することをLeetCodeのジャッジシステムが検知してRestrictions Failedするようになっている。
    > Use of .parse::<i32>() on the multi-character string 'numbers' violates the restriction against built-in string-to-number conversion functions.
    */

    pub fn my_atoi(s: String) -> i32 {
        let mut s_iter = s.trim_start().chars().peekable();
        let mut sign = 1;
        match s_iter.peek() {
            Some('+') => {
                s_iter.next();
            }
            Some('-') => {
                sign = -1;
                s_iter.next();
            }
            _ => (),
        };

        let number_characters = s_iter.take_while(|c| c.is_numeric()).collect::<String>();
        if number_characters.is_empty() {
            return 0;
        }

        let mut result = 0i32;
        for c in number_characters.chars() {
            let digit = c.to_digit(10).unwrap() as i32;
            if result > i32::MAX / 10 || result == i32::MAX / 10 && digit > i32::MAX % 10 {
                if sign == -1 {
                    return i32::MIN;
                }
                return i32::MAX;
            }

            result *= 10;
            result += digit;
        }

        sign * result
    }

    // オーバーフローチェックを組み込み関数に丸投げする実装
    pub fn my_atoi2(s: String) -> i32 {
        let mut s_iter = s.trim_start().chars().peekable();
        let mut sign = 1;
        match s_iter.peek() {
            Some('-') => {
                sign = -1;
                s_iter.next();
            }
            Some('+') => {
                s_iter.next();
            }
            _ => (),
        }
        let numbers = s_iter.take_while(|c| c.is_numeric()).collect::<String>();
        if numbers.is_empty() {
            return 0;
        }

        let get_i32_bounds = || -> i32 {
            if sign == -1 {
                return i32::MIN;
            }
            i32::MAX
        };
        let mut result = 0i32;
        for c in numbers.chars() {
            let Some(digit) = c.to_digit(10) else {
                unreachable!("numbers only numeric character");
            };
            let digit = digit as i32;
            let Some(num) = result.checked_mul(10) else {
                return get_i32_bounds();
            };
            let Some(num) = num.checked_add(digit) else {
                return get_i32_bounds();
            };
            result = num;
        }

        result.checked_mul(sign).unwrap_or_else(get_i32_bounds)
    }
}
