struct ZigzagConversion {}
impl ZigzagConversion {
    /*
    問題の理解:
    文字列sと整数num_rowsが与えられる。
    文字列sを整数num_rowsで指定された行数でジグザグパターンにして返す。
    s=PAYPALISHIRING, num_rows=3

    P   A   H   N
    A P L S I I G
    Y   I   R
    PAHNAPLSIIGYIR

    memo:
    0 ~ num_rows行毎に答えの文字列を保持する。
    0 ~ num_rowsまで走査しながら対応する行に文字をpush
    num_rows ~ 0まで走査しながら対応する行に文字をpush
    列の最初、最後のときに列の進む方向を切り替える。
    この手順をコードで表現できれば解けそう。

    この考え方で複数回Wrong Answerとなりながら、Acceptedになるコードを書けた。
    文字数とnum_rowsの関係で文字列をそのまま返すケースを見落としていた。
    NGとしておく。
    別の解法を写経しておく。
    */
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }

        let mut converted_s = vec![String::new(); num_rows as usize];
        let mut i = 0;
        let mut row = 0i32;
        let mut direction = 1i32;
        let s_chars = s.chars().collect::<Vec<_>>();
        while i < s.len() {
            converted_s[row as usize].push(s_chars[i]);
            row += direction;
            if row == 0 || row == num_rows - 1 {
                direction *= -1;
            }
            i += 1;
        }

        converted_s.concat()
    }

    /*
    - (0..num_rows).chain((1..num_rows - 1).rev())
      - [0,1,2,1]
      - 0 -> 1 -> 2 -> 1 (cycleにより先頭に戻る) -> 0 -> 1 -> 2 ...
    - sort_by_keyでindexのみを指定してソートすることで、元の文における相対的な文字の位置が保持されている
    chain,cycle,zipといったメソッドの実際の利用方法を見られるという観点では良い実装だと思う。
    */
    pub fn convert2(s: String, num_rows: i32) -> String {
        let mut zigzags = (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .collect::<Vec<_>>();
        zigzags.sort_by_key(|(row_index, _)| *row_index);
        zigzags.into_iter().map(|(_, c)| c).collect::<String>()
    }
}
