struct CapacityToShipPackagesWithinDDays {}
impl CapacityToShipPackagesWithinDDays {
    /*
    問題の理解:
    整数からなる配列weightsと整数daysが与えられる。
    weightsは荷物重量の一覧を表しており、この荷物を船に積んで全てdays以内に配送することを考える。
    days以内に全ての荷物を運べるように船の積載上限の最小値を計算して返す。
    weightsの並び順は変更することができない。

    memo:
    問題は理解できているが、どこから手を付けて良いのか分からないという感じがする。
    1日のとき、荷物を全て一度に積む必要があるので、weightsの合計値が答えとなる。
    weights = [1, 2, 3]
    2日のとき、3になる。sum(1, 2), sum(3)と考えているので累積和を見ている。
    手が止まったので答えを見る。

    解法の理解:
    - max_capacity_of_dayで荷物の総重量を表している。1日で全て運ぶ時。
    - min_capacity_of_dayで荷物の重さのうち、一番重い重量を表している。最低でもこの積載量が無いと運べないという下限。
    - min_capacity_of_day ~ max_capacity_of_day の範囲で二分探索している。
      - 範囲の中央値である一日あたりの積載量で試してみて、days以内に運べたかどうかで、答えが左右どちらの範囲にありそうかを探している。

    */
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut lower_capacity_of_day = *weights.iter().max().unwrap();
        let mut upper_capacity_of_day = weights.iter().sum();
        while lower_capacity_of_day < upper_capacity_of_day {
            let middle_capacity_of_day =
                lower_capacity_of_day + (upper_capacity_of_day - lower_capacity_of_day) / 2;
            let mut required_days = 1;
            let mut load_of_day = 0;
            for &weight in &weights {
                load_of_day += weight;
                if middle_capacity_of_day < load_of_day {
                    required_days += 1;
                    load_of_day = weight;
                }
            }

            if days < required_days {
                lower_capacity_of_day = middle_capacity_of_day + 1;
            } else {
                upper_capacity_of_day = middle_capacity_of_day;
            }
        }

        lower_capacity_of_day
    }
}
