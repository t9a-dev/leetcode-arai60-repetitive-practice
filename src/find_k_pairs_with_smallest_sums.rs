use std::{
    cmp::{Reverse, min},
    collections::BinaryHeap,
};

struct FindKPairsWithSmallestSums {}
impl FindKPairsWithSmallestSums {
    /*
    問題の理解:
    昇順ソートされた整数からなる配列nums1,nums2が与えられる。
    [nums1[i],nums2[j]]とペアを作り、ペアの合計値が少ない順の上位k件のペアを配列で返す。

    memo:
    最小順のペア上位k件を見つけたい。
    nums1[i],nums2[j]のi,jが0に近ければ近いほど小さなペアができる。昇順に並んでいるため。
    ナイーブな実装を考える。n = nums1.len, m = nums2.lenとすると時間計算量O(n*m)になる。
    問題の制約から10 ^ 10になる。10 ^ 10 / 10 ^ 8 = 約100秒となりTime Limit Exceededとなると思う。
    k < 10 ^ 4 なので早期リターンできそうだが、ペアの作り方を最小順になるようにしないと枝刈りできない。全てのペアを並べて見るまで最小順にできないので。
    与えられる配列は昇順になっているので、先頭からいい感じにペアを作ることができれば最小順のペアから生成できるのでk件に到達したら早期リターンできる。
    つまり時間計算量はO(k)になると思う。
    実装が思いつかないので答えを見る。

    解法の理解:
    nums1[0]とnums2の全てでペア(0,j)を作って最小ヒープに入れる。
    最小ヒープからk回ペアを取り出す。
    ペアを取り出すたびに、(0,j+1)のように最小のペアの候補となるペアを作って最小ヒープにpushする。
    nums1,nums2のより先頭に近い最小の値になりえる値のペアを最小ヒープに追加できる。

    */
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = min(k as usize, nums1.len() * nums2.len());
        let mut smallest_pairs_by_sum: BinaryHeap<(Reverse<i32>, (usize, usize))> =
            BinaryHeap::new();
        for (i, num1) in nums1.iter().enumerate() {
            let sum = num1 + nums2[0];
            smallest_pairs_by_sum.push((Reverse(sum), (i, 0)));
        }

        let mut top_k_smallest_pairs = vec![];
        while let Some((_, (i, j))) = smallest_pairs_by_sum.pop() {
            if top_k_smallest_pairs.len() == k {
                break;
            }
            top_k_smallest_pairs.push(vec![nums1[i], nums2[j]]);

            let next_j = j + 1;
            let Some(num2) = nums2.get(next_j) else {
                continue;
            };
            let sum = nums1[i] + num2;
            smallest_pairs_by_sum.push((Reverse(sum), (i, next_j)));
        }

        top_k_smallest_pairs
    }
}
