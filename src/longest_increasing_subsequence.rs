struct LongestIncreasingSubsequence {}
impl LongestIncreasingSubsequence {
    /*
    問題の理解:
    整数配列numsが与えられるので、厳密に増加する部分配列の最大の長さを返す。
    部分配列とは元の配列から要素の一部またはすべてを削除することで得られる配列のことで、残りの要素の並び順は変更しない。

    memo:
    配列長さが0のときは長さ0
    配列長さ1のときは長さ1
    配列長さ2のときは長さ1or2
    配列長さ3のときは長さ1or2or3
    配列長さ4のときは長さ1or2or3or4
    ...
    と続く。
    base caseは配列長さ1のとき1となる部分だと思う。
    最小問題はnums[i] < nums[i+1] または nums[i-1] < nums[i]の形で表せると思う。
    最長長さの情報を引き継ぎたい。
    手が止まったので答えを見る。

    解法の理解:
    dp[i]はnums[i]を最後にしたときの、増加部分配列長
    nums[i]はそれ自体が部分列として長さ1を持つ。初期値として長さ1として考えられる
    [2, 3, 1]
    [2, 3, 1, 4]
    nums[i] < nums[j] then dp[j] = dp[j].max(dp[i] + 1)

    LLMに壁打ちして解法の理解をして実装自体は自力でAcceptedとなった。
    */
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        };
        let mut increasing_subsequences = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] >= nums[j] {
                    continue;
                }
                increasing_subsequences[j] =
                    increasing_subsequences[j].max(increasing_subsequences[i] + 1);
            }
        }

        *increasing_subsequences.iter().max().unwrap()
    }

    /*
    FollowUpで時間計算量をO(n log n)にできるかというものがある。
    元の配列の並び順を情報として失わないように、value to index のHashMapを作っておく。
    ソートする。O(n log n)
    ソート済なので二分探索木が可能になる。O(log n)
    ここで手が止まったのでフォローアップの解法を見る。

    解法の理解:
    numsを先頭から順に走査しながら、部分列subを作る。
    部分列subの不変条件として昇順であることとする。
    部分列subに加えるnums[i]の挿入位置を二分探索で探す。二分探索の条件を sub[x] < nums[i] とする。
    不変条件としてsub[i]は、長さi+1の増加部分列における末尾の最小値となる。

    問題のカテゴリが動的計画法だという先入観に囚われすぎて、まずDPテーブルを作ろうとしている感じがある。
    */
    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        };
        let mut increasing_subsequence = Vec::new();
        for num in nums {
            let increasing_insert_position = increasing_subsequence.partition_point(|v| *v < num);

            if increasing_subsequence.len() <= increasing_insert_position {
                increasing_subsequence.push(num);
                continue;
            }
            increasing_subsequence[increasing_insert_position] = num;
        }

        increasing_subsequence.len() as i32
    }
}
