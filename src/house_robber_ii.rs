use std::collections::HashMap;

struct HouseRoberII {}
impl HouseRoberII {
    /*
    問題の理解:
    各家にまとまった金額のお金が置いてある。家は環状に配置されている。
    整数配列numsが与えられる。各家に置いてある金額が表されている。
    各家から金を盗んだ時に利益が最大となる金額を返す。
    制約として隣り合った家からは盗むことができない。
    環状に家が並んでいるので、nums.first() ,nums.last()は隣り合っていることになる。

    memo:
    最初の家からnums[0]スタートすることを考える。
    dp[0]からスタートすることになるが、家が環状に並んでおり、dp[i-1].max(nums[i] + dp[i-2])のdp[i-1],dp[i-2]が計算済でないのでこの方法では計算できないように見える。
    最初の状態が決まっている必要があるため。
    手が止まったので答えをみる。

    解法の理解:
    nums[0]とnums[nums.len - 1]が隣接していることで、dp[i]を求める時に利用する状態に隣接する家が含まれていないという制約を満たせなくなる。
    nums[0],nums[nums.len - 1]をそれぞれ除外した状態を作ることで、開始地点と終了地点が循環しない状態を作り、隣接する家の値を使わないという条件を満たせるようにする。
    つまり、直線に並んだ家の状態を２つ作ってそれぞれで計算して、最終的にこれら２つの状態からさらに利益の大きい方を解として返す。
    */
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => return 0,
            1 => return *nums.first().unwrap(),
            2 => return nums[0].max(nums[1]),
            _ => (),
        }

        let without_first_house = &nums[1..];
        let without_last_house = &nums[..nums.len() - 1];

        Self::explore_max_amount(
            &mut HashMap::new(),
            without_first_house,
            without_first_house.len() as isize - 1,
        )
        .max(Self::explore_max_amount(
            &mut HashMap::new(),
            without_last_house,
            without_last_house.len() as isize - 1,
        ))
    }

    fn explore_max_amount(amount_cache: &mut HashMap<isize, i32>, nums: &[i32], i: isize) -> i32 {
        if i < 0 {
            return 0;
        }
        if let Some(amount) = amount_cache.get(&i) {
            return *amount;
        }

        let amount = Self::explore_max_amount(amount_cache, nums, i - 1)
            .max(nums[i as usize] + Self::explore_max_amount(amount_cache, nums, i - 2));
        amount_cache.insert(i, amount);

        amount
    }

    pub fn rob2(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => return 0,
            1 => return *nums.first().unwrap(),
            2 => return nums[0].max(nums[1]),
            _ => (),
        }

        let explore_max_amount = |nums: &[i32]| -> i32 {
            let mut robbed = vec![nums[0], nums[0].max(nums[1])];
            for i in 2..nums.len() {
                robbed.push(robbed[i - 1].max(nums[i] + robbed[i - 2]));
            }
            *robbed.last().unwrap()
        };
        let max_amount_without_first = explore_max_amount(&nums[1..]);
        let max_amount_without_last = explore_max_amount(&nums[..nums.len() - 1]);

        max_amount_without_first.max(max_amount_without_last)
    }
}
