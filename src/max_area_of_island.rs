struct MaxAreaOfIsland {}
impl MaxAreaOfIsland {
    /*
    問題の理解:
    m行n列からなるgridが与えられる。
    1を陸、0を水域とし、陸が上下左右につながっているものを島とする。島の面積のうち最大値を求めて返す。島の面積とは陸(1)の数。
    島が存在しないときは0を返す。

    memo:
    1を見つけたら、面積のカウントを開始して、陸を見つけるたびにインクリメントする。一度見た陸は水域に書き換えて重複して見ないようにする。
    引数のシグネチャからgridの所有権ごと移動している(可変参照ではない)ので書き換えても呼び出し元に影響を与えないので問題ない。

    Accepted
    */
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 0 {
                    continue;
                }
                let mut explored_area = 0;
                Self::explore_island(&mut grid, x, y, &mut explored_area);
                max_area = max_area.max(explored_area);
            }
        }

        max_area
    }

    fn explore_island(grid: &mut [Vec<i32>], x: usize, y: usize, explored_area: &mut i32) {
        let Some(rows) = grid.get_mut(y) else {
            return;
        };
        let Some(v) = rows.get_mut(x) else {
            return;
        };
        if *v == 0 {
            return;
        }

        *v = 0;
        *explored_area += 1;

        if 0 < x {
            Self::explore_island(grid, x - 1, y, explored_area);
        }
        if 0 < y {
            Self::explore_island(grid, x, y - 1, explored_area);
        }
        Self::explore_island(grid, x + 1, y, explored_area);
        Self::explore_island(grid, x, y + 1, explored_area);
    }
}
