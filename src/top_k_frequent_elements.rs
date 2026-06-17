use std::collections::{BinaryHeap, HashMap};

struct TopKFrequentElements {}
impl TopKFrequentElements {
    /*
    問題の理解:
    整数配列nums,整数kが与えられるので、k個の最も頻繁に出現する要素を配列で返す。
    戻り値の配列の並び順は任意で良い。

    memo:
    HashMapで要素の数を数え上げる
    最大ヒープに出現数をキーとして要素を突っ込む
    k回popして配列に詰めて返す

    自力で解けた
    */
    fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num_to_frequency = HashMap::new();
        nums.into_iter().for_each(|num| {
            num_to_frequency
                .entry(num)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        });

        let mut max_heap_by_frequency = BinaryHeap::from_iter(
            num_to_frequency
                .into_iter()
                .map(|(num, frequency)| (frequency, num)),
        );
        let mut top_k_frequency_nums = Vec::new();
        for _ in 0..k {
            if let Some((_, num)) = max_heap_by_frequency.pop() {
                top_k_frequency_nums.push(num);
            }
        }

        top_k_frequency_nums
    }
}
