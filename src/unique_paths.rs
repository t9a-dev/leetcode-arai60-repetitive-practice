use std::collections::HashMap;

struct UniquePaths {}
impl UniquePaths {
    /*
    問題の理解:
    ロボットがmxn上のグリッドの左上に配置されているgrid[0][0]
    ロボットは最終的に右下隅grid[m-1][n-1]へ移動しようとする。
    ロボットはある時点で下方向または右方向のいずれか一方にのみ移動可能。
    2つの整数m,nが与えられた時、ロボットが右下隅に移動するために取れるユニークな経路の総数を返す。

    memo:
    DFSで右下に到達したら1を返すような再帰処理を実装する。
    2通りの選択肢(右or下)がある。
    一度到達した座標をHashMapでメモ化してキャッシュする。
    右下からスタートすると考えて、座標が(0,0)に到達するという考え方のほうがシンプルそう。
    base casedで(0,0)に到達したら1を返す。
    この場合は左、上に行くと考える。

    Time Limit Exceededとなった。メモ化の方法を見直す。

    キャッシュへの追加漏れを修正してAccepted。一度Wrong Answerとなったのと、全体で1時間程度かかったのでNGとする。

    メモ化再帰処理による解法は書けたが、dpテーブルのような動的計画法の解法も写経しておく。
    */
    fn unique_paths(m: i32, n: i32) -> i32 {
        let mut visited_paths: HashMap<(i32, i32), i32> = HashMap::new();
        Self::explore_unique_path(&mut visited_paths, m - 1, n - 1)
    }

    fn explore_unique_path(visited_paths: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) -> i32 {
        if x < 0 || y < 0 {
            return 0;
        }
        if x == 0 && y == 0 {
            return 1;
        }
        let to_left_paths = match visited_paths.get(&(x - 1, y)) {
            Some(paths) => *paths,
            None => Self::explore_unique_path(visited_paths, x - 1, y),
        };
        visited_paths.insert((x - 1, y), to_left_paths);

        let to_up_paths = match visited_paths.get(&(x, y - 1)) {
            Some(paths) => *paths,
            None => Self::explore_unique_path(visited_paths, x, y - 1),
        };
        visited_paths.insert((x, y - 1), to_up_paths);

        to_left_paths + to_up_paths
    }

    fn explore_unique_path2(visited_paths: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) -> i32 {
        if let Some(paths) = visited_paths.get(&(x, y)) {
            return *paths;
        };
        if x < 0 || y < 0 {
            return 0;
        }
        if x == 0 && y == 0 {
            return 1;
        }

        let paths = Self::explore_unique_path2(visited_paths, x - 1, y)
            + Self::explore_unique_path2(visited_paths, x, y - 1);
        visited_paths.insert((x, y), paths);

        paths
    }

    /*
    動的計画法による解法。
    ロボットが左上grid[0][0]にいるので、経路数1と考えられる。
    grid[0][x], grid[y][0]の経路数は1になる。それぞれ直線の経路なので。
    前の経路は1つ左または1つ上のみ。
    ある位置grid[y][x]へつながる経路の数はgrid[y-1][x] or grid[y][x-1]として求められる。
    求めたいのは経路の総数なので、grid[y-1][x] + grid[y][x-1]となる。
    */
    fn unique_paths2(m: i32, n: i32) -> i32 {
        if m < 0 || n < 0 {
            return 0;
        }
        let (x, y) = (m as usize, n as usize);

        let mut grid = vec![vec![1; x]; y];
        for row in 1..y {
            for col in 1..x {
                grid[row][col] = grid[row - 1][col] + grid[row][col - 1];
            }
        }

        grid[y - 1][x - 1]
    }
}
