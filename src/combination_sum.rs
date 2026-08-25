struct CombinationSum {}
impl CombinationSum {
    /*
    問題の理解:
    重複のない整数からなる配列nums,整数targetが与えられる。
    candidatesから選んだ数値の合計がtargetと等しくなるユニークな集合を返す。
    candidatesに含まれる数値は何回でも利用可能。

    memo:
    問題を見てもすぐに解法が思いつかないので、決定木で考えてみる。
    candidatesの値は何回でも利用できるので、決定木で分岐しても候補は変わらない。
    remaining = target - candidates[i] とするとremainingは決定木が分岐するたびに変わる。
    remaining = 0となるパスで選んだ値の集合を答えとして扱える。
    重複がでないようにどうすればよいか。
    手が止まったので答えを見る。

    解法の理解:
    candidatesから1つの要素を選んで、targetとなるような合計値を作るのに使えそうかを確認する。
    使えそうなら候補としてcombinationにpushして再帰に入る。
    sum == targetのときにcombinationを答えの集合(combinations)に入れる。
    combinationに値を入れて再帰に入ったら、次の候補を試す前にcombination.pop()する。この部分がbacktrackingになっている。

    所感:
    ある候補を選んだときに、選んだ候補を取り除いた残りの候補をさらに再帰で見ていくという点が思いつかなかった。
    解法のコードを読んで、このアルゴリズムだと重複しないことが理解できたが、重複しないようにすると考えた時にこの手順が思いつかないなという感覚。

    */
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combinations = Vec::new();
        let mut combination = Vec::new();
        Self::make_combination(&candidates, 0, target, &mut combination, &mut combinations);

        combinations
    }

    fn make_combination(
        candidates: &[i32],
        sum: i32,
        target_sum: i32,
        combination: &mut Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if sum == target_sum {
            combinations.push(combination.to_vec());
            return;
        }

        let Some((first_candidate, remaining_candidates)) = candidates.split_first() else {
            return;
        };
        let candidate_sum = sum + first_candidate;
        if candidate_sum <= target_sum {
            combination.push(*first_candidate);
            Self::make_combination(
                candidates,
                candidate_sum,
                target_sum,
                combination,
                combinations,
            );
            combination.pop();
        }

        Self::make_combination(
            remaining_candidates,
            sum,
            target_sum,
            combination,
            combinations,
        );
    }
}
