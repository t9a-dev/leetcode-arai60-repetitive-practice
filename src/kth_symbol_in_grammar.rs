struct KthSymbolInGrammar {}
impl KthSymbolInGrammar {
    /*
    問題の理解:
    n行からなるテーブルを作成し、1行目に0を設定する。ここで行は1-indexedつまり、1から始まる。
    それ以降の各行で他の行を参照し、0が現れる箇所を全て01、1が現れる箇所を10に置き換える。
    2つの正数n,kが与えられた時、n行からなるテーブルのn行目におけるk番目のシンボルを返す。

    memo:
    最初の状態が決まっていて、そこから遷移していくので、動的計画法みたいな感じがする。
    ただし、入力の制約でkは2 ^ n - 1 となり、nは30までなので状態を全て保持していると空間計算量が爆発すると思う。
    今の状態を1つ前の状態から計算しつつ最後の状態を1つの変数で管理する方向で良さそう。
    ビット操作で解けそうだが、操作の仕方がよく分からない。
    1: 0
    2: 01
    3: 0110
    4: 01101001

    手が止まったので答えを見る。

    解答の理解:
    決定木として考える
    n = 3, k = 4
        0
       / \
      0   1 <- parent_value=1
     / \ / \
    0  1 1  0 <- k=4

    base case n == 1 return 0となる。
    親が0のとき、子は[0,1]の並びになり、k + 1 % 2をindexとして考えられる。
    親が1のとき、子は[1,0]の並びになり、k + 1 % 2をindexとして考えられる。
    nは単調減少するのでn - 1で良い。
    kは単純に2で割ると、1つ上の層に移動した時に、親が変わってしまう。
    n = 3, k = 3の親は n = 2, k = 2となるべきだが、 k = k / 2 = 1 となり正しいkの位置が得られない。
    k = k - (k / 2) = 3 - 3 / 2 = 2 とすることで、n = 2, k = 2と正しい位置が得られる。
    */

    const BINARY_TREE_ROOT_VALUES: [[i32; 2]; 2] = [[0, 1], [1, 0]];
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        let parent_value = Self::kth_grammar(n - 1, k - (k / 2));
        Self::BINARY_TREE_ROOT_VALUES[parent_value as usize][((k + 1) % 2) as usize]
    }

    // bit操作による解法
    pub fn kth_grammar_2(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        // n行目のbit列はn-1行目のbit列,n-1行目のbit列を反転したもので構成されている。
        // n = 3のときhalf_length = 2(0010)となる。
        // n = 3のときのbit列0110の前後の境界を求めている。
        let half_length = 1 << (n - 2);
        // kがhalf_length内に収まっていれば、答えは前半部分にある。
        if k <= half_length {
            return Self::kth_grammar_2(n - 1, k);
        }
        // kがhalf_lengthを超える時、答えは後半部分にある。
        // k - half_lengthでn-1行目における対応する位置を求める。
        // 後半部分はn-1行目の値を反転したbit列なので、 1- bit列としてn-1行目のbit列を反転する。
        1 - Self::kth_grammar_2(n - 1, k - half_length)
    }
}
