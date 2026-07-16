use std::collections::HashMap;

struct UniquePathsII {}
impl UniquePathsII {
    /*
    問題の理解:
    m * nの整数配列obstacle_gridが与えられる。
    ロボットは左上に配置されており、最終的に右下を目指して移動する。
    ロボットは任意の時点で、下方向または右方向どちらかに進むことができる。
    与えられた配列で、障害物が1、空きスペースは0として表現されている。
    障害物がある経路をロボットが通ることはできない。
    ロボットが右下に到達するために利用できる一意な経路の数を返す。

    memo:
    ロボットの初期位置grid[0][0]の経路数を1とする。
    動的計画法でgrid[y][x] = grid[y-1][x] + grid[y][x-1]で求めることができる。
    obstacle_gridに対応する座標に障害物があれば、経路をゼロとして扱う。
    初期状態として、grid[0][x], grid[y][0]を経路1として扱えないことに注意が必要そう。
    ここに障害物がある可能性がある。
    grid[y][0],grid[0][x]には、それぞれobstacle_gridの対応する座標の値を反転した値を入れれば良さそう。
    障害物1として表現されているので、反転して経路数0として表せる。空きスペースは経路数1として表現できる。

    Wrong Answerとなった。分からないので答えをみる。

    解法の理解:
    obstacle_gridと同じサイズのgridを用意する。
    1行目、1列目について、障害物を見つけるまで1で書き換える。障害物を見つけたら、それ以降は到達できない経路なので書き換え作業を中止してい0のままにしておく。
    2行目,2列目から走査しつつ、対応する座標に障害物が無いかを確認しながら動的計画法でパスの数を数え上げる。
    障害物を見つけたらcontinueする。(初期値0のままにしておくことで経路の数が無いことがわかる。)
    */
    const OBSTACLE: i32 = 1;
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let rows = obstacle_grid.len();
        let cols = match obstacle_grid.first() {
            Some(cols) => cols.len(),
            None => 0,
        };
        let mut grid = vec![vec![0; cols]; rows];

        // first row
        for x in 0..cols {
            let has_obstacle = obstacle_grid[0][x] == Self::OBSTACLE;
            if has_obstacle {
                break;
            }
            grid[0][x] = 1;
        }
        // first col
        for y in 0..rows {
            let has_obstacle = obstacle_grid[y][0] == Self::OBSTACLE;
            if has_obstacle {
                break;
            }
            grid[y][0] = 1;
        }

        for y in 1..rows {
            for x in 1..cols {
                let has_obstacle = obstacle_grid[y][x] == Self::OBSTACLE;
                if has_obstacle {
                    continue;
                }
                grid[y][x] = grid[y][x - 1] + grid[y - 1][x];
            }
        }

        grid[rows - 1][cols - 1]
    }

    pub fn unique_paths_with_obstacles2(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let rows = obstacle_grid.len();
        let cols = match obstacle_grid.first() {
            Some(col) => col.len(),
            None => 0,
        };
        let mut visited_paths = HashMap::new();
        Self::explore_paths(
            &mut visited_paths,
            &obstacle_grid,
            cols as i32 - 1,
            rows as i32 - 1,
        )
    }

    fn explore_paths(
        visited_paths: &mut HashMap<(i32, i32), i32>,
        obstacle_grid: &[Vec<i32>],
        x: i32,
        y: i32,
    ) -> i32 {
        if let Some(cached_path) = visited_paths.get(&(x, y)) {
            return *cached_path;
        }
        if x < 0 || y < 0 {
            return 0;
        }

        let has_obstacle = obstacle_grid[y as usize][x as usize] == Self::OBSTACLE;
        if has_obstacle {
            return 0;
        }
        if x == 0 && y == 0 {
            return 1;
        }

        let paths = Self::explore_paths(visited_paths, obstacle_grid, x - 1, y)
            + Self::explore_paths(visited_paths, obstacle_grid, x, y - 1);
        visited_paths.insert((x, y), paths);

        paths
    }
}
