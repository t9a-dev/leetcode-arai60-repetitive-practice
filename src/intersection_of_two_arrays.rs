use std::collections::HashSet;

struct InterSectionOfTwoArrays {}

impl InterSectionOfTwoArrays {
    /*
    問題の理解:
    整数からなる配列nums1,nums2が与えられるので、これら2つの配列両方に含まれる整数を配列にして返す。

    memo:
    intersectionメソッドがあるのでこれを使うだけで済むが、練習としてこれを使わない方向でまず書く。
    nums1,nums2の内サイズの小さい方を利用して、HashSetに値を入れておく。
    nums1,nums2のサイズに極端に偏りがある場合にHashSetに入れる補助空間計算量を定数倍ではあるが削減するため。
    HashSetで利用しなかった方を走査しつつ、すでにHashSetに存在する値を見つけたら解のHashSetにpushしていく。

    一度WrongAnswerとなった。
    intersectionsの解を入れるのをHashSetではなく、配列にしていて重複する値を返してしまった。

    配列の短い方をHashSetに入れることで、補助空間計算量を削減したものの、定数倍で見るのであれば時間計算量が悪化する（ループの回数が増える）ので必ずこちらが良いとも限らない。
    ただ、Hash化のコストも考えると短い配列の方を入れる方が良さそうなので、どちらか選ぶのであれば短い方の配列を最初にHashSetに入れると思った。

    関数引数のシグネチャをmutにしているが、元々Vec<i32>と参照ではなく、所有権をmoveしているので、呼び出し側に影響は無いと考える。
    */
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        if nums2.len() < nums1.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }

        let nums_set: HashSet<_> = HashSet::from_iter(nums1);
        let mut intersections = HashSet::new();
        for num2 in nums2 {
            nums_set.contains(&num2).then(|| intersections.insert(num2));
        }

        intersections.into_iter().collect()
    }

    // ワンライナーでの書き方が分からなかった。
    pub fn intersection2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let a: HashSet<_> = HashSet::from_iter(nums1);
        let b: HashSet<_> = HashSet::from_iter(nums2);
        a.intersection(&b).copied().collect()
    }

    // ワンライナー
    // 最初のHashSet(nums1)で型アノテーション付ける必要がある
    pub fn intersections3(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        HashSet::<_>::from_iter(nums1)
            .intersection(&HashSet::from_iter(nums2))
            .copied()
            .collect()
    }
}
