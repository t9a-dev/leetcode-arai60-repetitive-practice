use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    sum: i32,
    values: Vec<i32>,
}

impl Pair {
    pub fn new(values: Vec<i32>) -> Self {
        Self {
            sum: values.iter().sum(),
            values,
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum.cmp(&other.sum)
    }
}

struct FindKPairsWithSmallestSum {}
impl FindKPairsWithSmallestSum {
    /*
    問題の理解:
    整数からなる配列nums1,nums2が与えられる。これらの配列は昇順ソートされている。
    nums1,nums2の要素をペアとして、この時の合計値が小さいペアk個を返す。

    memo:
    nums1,nums2は昇順ソートされているので、なるべく合計値が小さくなるようにペアを作りながら、kに達したら早期リターンできそう。
    早期リターンするには最小のペアから作る必要があり、ここのやり方が分からない。
    ナイーブな実装としては、合計値をキー、ペアを値として、最小ヒープに入れていく
    k回popして答えとして返す。

    WronAnswerとなり、解けなかったので写経する
    - Heapから値を取り出す時にpop()ではなく、iter().take(k)として最小の値を取り出せていなかった
    - Memory Limit Exceededとなった。
        - 全てのペアを作ろうとしているので、入力の制約から10 ^ 10を扱うことになる。

    */
    pub fn k_smallest_pairs_wrong_answer(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        k: i32,
    ) -> Vec<Vec<i32>> {
        //////////////////
        // WrongAnswer
        //////////////////
        let mut min_heap_by_pairs_sum = BinaryHeap::new();
        for num1 in nums1.iter() {
            for num2 in nums2.iter() {
                min_heap_by_pairs_sum.push(Reverse(Pair::new(vec![*num1, *num2])));
            }
        }

        let mut smallest_pairs = Vec::new();
        for _ in 0..k as usize {
            if let Some(Reverse(pair)) = min_heap_by_pairs_sum.pop() {
                smallest_pairs.push(pair.values.clone());
            }
        }

        smallest_pairs
    }

    /*
    解法の理解:
    nums1のすべての要素、nums2[0]の要素でペアを作って合計を計算し最小ヒープに入れる。このとき要素の位置をペアとして保持しておく。
    以下の手順を繰り返す
    - 最小ヒープから値を取り出して、戻り値の配列に追加
    - 戻り値の配列のサイズとkが等しければループを終了して戻り値の配列を返す
    - 最小ヒープにnums1[i], nums2[j+1]からなるペアを追加する
        - 最初にnums1[i], nums2[0]で固定したペアを生成しており、nums1[i], nums2[1]のペアの合計値も確認する必要がある。

    この手順だと全てのペアの組み合わせを見ずとも、上位k件の最小値ペアを取り出すことができる。
    全ての配列の組み合わせを計算する必要があるのか、k件まで見ればよいのかを
    min(nums1.len() * nums2.len(), k)
    で判定している

    以下の点が重要だと思った。
    - O(n ^ 2)となるような全てのペアをナイーブに計算しない
        - 全てのペアを計算せずに必要な分(上位k件)だけ計算して、必要ない分は計算しなくて済むにはどうするかを考える
    - nums1, nums2が昇順ソートになっていることに注目する
    */
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut smallest_by_sum = BinaryHeap::new();
        for (i, num1) in nums1.iter().enumerate() {
            // nums1が長いときに全て計算するのではなく、最大でもkの長さに抑えておく。
            // nums1, nums2 ともに昇順なので配列先頭側に小さい数字が集まっている。
            // 最終的にはnums2の先頭からsmallest_by_sumに差し込むのでnums1の末尾よりの数値まで最初から使う必要がない。
            if k < i {
                break;
            }

            let sum = num1 + nums2[0];
            smallest_by_sum.push((Reverse(sum), (i, 0)));
        }

        let mut top_k_smallest_pairs = Vec::new();
        while let Some((_, (i, j))) = smallest_by_sum.pop() {
            if top_k_smallest_pairs.len() == k {
                break;
            }

            top_k_smallest_pairs.push(vec![nums1[i], nums2[j]]);

            let next_j = j + 1;
            if let Some(num2) = nums2.get(next_j) {
                let sum = nums1[i] + num2;
                smallest_by_sum.push((Reverse(sum), (i, next_j)));
            }
        }

        top_k_smallest_pairs
    }
}

#[cfg(test)]
mod tests {
    use crate::find_k_pairs_with_smallest_sum::FindKPairsWithSmallestSum;

    #[test]
    fn k_smallest_pairs_test() {
        let smallest_pairs =
            FindKPairsWithSmallestSum::k_smallest_pairs(vec![1, 2, 4, 5, 6], vec![3, 5, 7, 9], 3);
        assert_eq!(vec![vec![1, 3], vec![2, 3], vec![1, 5]], smallest_pairs);
    }
}
