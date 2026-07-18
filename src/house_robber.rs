use std::{collections::HashMap, hash::Hash, ops::Sub};

struct HouseRobber {}
impl HouseRobber {
    /*
    問題の理解:
    金額を表す整数配列numsが与えられる。
    複数の家があり、各家に置いてある金額を表している。
    制約として隣り合う連続した家からは窃盗できないという制約の中で、最大の金額となるように窃盗を行ったときの金額を返す。

    memo:
    マイナスの金額は存在しない。
    選択肢として、今nums[i]盗むか、次nums[i+1]盗むかの分岐がある。制約として隣り合う位置の家から窃盗することができないため。
    dp[i]はその時点の累積和と考えられそう。
    dp[i] = dp[i-2] + nums[i]
    dp[i]の最大値を返す。

    Wrong Answerとなった。最初と最後の組み合わせが最大値になる組み合わせに対応できていない。

    答えを見る。
    WrongAnswerとなった実装では常に1つ飛ばした累積和を計算している。
    初期状態としてdp[0] = nums[0], dp[1] = nums[0].max(nums[1])としている。
    dp[1]に関してなぜこうなっているのか理解するのに時間がかかった。
    dpはある時点で盗むことができる最大の金額を持っている。
    制約から隣接する両方の家から盗むことができない。
    つまり、0番目の家と1番目の家どちらか一方からしか盗むことができない状況では、1番目の家を見ている状態ではnums[0].max(nums[1])を必ず持つことになる。
    隣り合う家から盗むことができないのと最大の金額を求めたいため。

    あるときの利益を最大化したいが、制約により連続する家から盗めない。
    - 今見ている家nums[i] + 2つ前の家までの合計robbed_money[i-2]
    - 直前までの合計値robbed_money[i-1]
        - 直前の家を含む合計値を見ているので、今見ている家nums[i]は盗めない

    */
    pub fn rob_wrong_answer(nums: Vec<i32>) -> i32 {
        /*
         * Wrong Answer
         */
        let mut robbed_money = vec![0; nums.len()];
        for i in 0..nums.len() {
            if 2 <= i {
                robbed_money[i] = nums[i] + robbed_money[i - 2];
                continue;
            }
            robbed_money[i] += nums[i];
        }

        *robbed_money.iter().max().unwrap()
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return *nums.first().unwrap();
        }
        let mut robbed_money = vec![nums[0], nums[0].max(nums[1])];
        for i in 2..nums.len() {
            robbed_money.push(robbed_money[i - 1].max(nums[i] + robbed_money[i - 2]));
        }

        *robbed_money.last().unwrap()
    }

    pub fn rob2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut position_to_money: HashMap<isize, i32> = HashMap::new();
        Self::run_robber(&nums, &mut position_to_money, nums.len() as isize - 1)
    }

    fn run_robber(
        nums: &[i32],
        position_to_money: &mut HashMap<isize, i32>,
        position: isize,
    ) -> i32 {
        if position < 0 {
            return 0;
        }
        if let Some(money) = position_to_money.get(&position) {
            return *money;
        };

        let robbed_money = Self::run_robber(nums, position_to_money, position - 1)
            .max(nums[position as usize] + Self::run_robber(nums, position_to_money, position - 2));
        position_to_money.insert(position, robbed_money);

        robbed_money
    }
}
