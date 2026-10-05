use std::collections::HashSet;

struct NumberOfIslands {}
impl NumberOfIslands {
    /*
     *
     * 問題の理解:
     * 陸地('1')と水域('0')を表すm*nの二次元グリッドが与えられた時、島の数を返す。
     * 隣接する陸地が水平または垂直方向に繋がっているものを島とする。グリッド外側は全て水域とする。
     *
     * memo:
     * 陸地を見つけたら島のカウントをインクリメントする。再帰的に水平、垂直方向に存在する陸地を辿る。たどった陸地は水域に書き換える。
     * 一度訪れた座標からは探索を開始しないようにメモ化する。
     * 島の数を数えるタイミングが怪しい。
     * '1'を見つけるたびにインクリメントすると陸地の数を数えてしまう。
     * '1'を見つけた再帰の開始時点でdetect_islandみたいなフラグをtrueにして、このフラグが立っている間はインクリメントしない。
     * '0'を見つけたらフラグをfalseにする。
     * 時間切れなのでコードを提出する。
     * Wrong Answerとなった。
     * 答えを見る。
     *
     * 解法の理解:
     * 単純に全ての座標をループで走査する。
     * '1'を見つけたら島の数をインクリメントする。垂直、左右方向で`1`がなくなるまで走査しながら、'0'に書き換える。
     * 次の座標を見に行く。
     */
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let Some(v) = Self::get_grid_value(&mut grid, x, y) else {
                    continue;
                };
                if *v == '0' {
                    continue;
                }

                Self::explore_islands(&mut grid, x, y);
                islands_count += 1;
            }
        }

        islands_count
    }

    fn get_grid_value(grid: &mut [Vec<char>], x: usize, y: usize) -> Option<&mut char> {
        grid.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn explore_islands(grid: &mut [Vec<char>], x: usize, y: usize) {
        let Some(v) = Self::get_grid_value(grid, x, y) else {
            return;
        };
        if *v == '0' {
            return;
        }
        *v = '0';

        if 0 < x {
            Self::explore_islands(grid, x - 1, y);
        }
        if 0 < y {
            Self::explore_islands(grid, x, y - 1);
        }
        Self::explore_islands(grid, x + 1, y);
        Self::explore_islands(grid, x, y + 1);
    }
}
