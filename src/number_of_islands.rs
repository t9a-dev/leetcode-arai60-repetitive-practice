use std::collections::VecDeque;

struct NumberOfIslandsBfs {}
impl NumberOfIslandsBfs {
    /*
    m行n列からなる二次元配列であるgridが与えられる。
    配列には陸を表す'1'、水域を表す'0'が入っている。
    島の数を返す。
    島とは 水平または垂直方向に陸同士がつながっており、周囲を水域が囲っている領域のこと。
    グリッドの外側はすべて水域である。

    memo:
    陸を見つけたら、水域に書き換えて次(i+1,j+1)に進んでいく
    水域を見つけたら探索を中止
    この手順を再帰でかけそうだが、島としてカウントする方法が思いつかず手が止まったので写経する。

    解法の理解:
    gridをループですべて見る。
    陸を見つけた時点で島のカウントをインクリメントする。陸から水域を見つけるまでBFS,DFSで陸を見つけるたびに水域に変更する。
    以下の操作に分けて考えると良さそう。
    - すべての座標を走査する
    - 上下左右行けるところまで行く
    - 一度訪れた座標は記録しておき、スキップする
        - Rustでは引数シグネチャから所有権の移動が行われているので、訪問済の座標を'0'に書き換えても問題ない。
        - 可変共有参照であっても、シグネチャから破壊的な変更がされることが読み取れるので変更しても問題なさそう。
    */
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut island_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '0' {
                    continue;
                }
                grid[y][x] = '0';
                Self::explore_islands(&mut grid, x, y);
                island_count += 1;
            }
        }

        island_count
    }

    fn explore_islands(grid: &mut [Vec<char>], start_x: usize, start_y: usize) {
        let mut visited_locations: VecDeque<(usize, usize)> = VecDeque::new();
        visited_locations.push_back((start_x, start_y));

        while let Some((x, y)) = visited_locations.pop_front() {
            if 0 < x {
                Self::visit_island(grid, x.saturating_sub(1), y, &mut visited_locations);
            }
            if 0 < y {
                Self::visit_island(grid, x, y.saturating_sub(1), &mut visited_locations);
            }
            Self::visit_island(grid, x + 1, y, &mut visited_locations);
            Self::visit_island(grid, x, y + 1, &mut visited_locations);
        }
    }

    fn visit_island(
        grid: &mut [Vec<char>],
        x: usize,
        y: usize,
        visited_locations: &mut VecDeque<(usize, usize)>,
    ) {
        let Some(rows) = grid.get_mut(y) else {
            return;
        };
        let Some(v) = rows.get_mut(x) else {
            return;
        };
        if *v == '0' {
            return;
        }

        *v = '0';
        visited_locations.push_back((x, y));
    }
}

struct NumberOfIslandsDfs {}
impl NumberOfIslandsDfs {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == '0' {
                    continue;
                }
                islands_count += 1;
                Self::explore_islands(&mut grid, x, y);
            }
        }

        islands_count
    }

    fn explore_islands(grid: &mut [Vec<char>], x: usize, y: usize) {
        let Some(rows) = grid.get_mut(y) else {
            return;
        };
        let Some(v) = rows.get_mut(x) else {
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
