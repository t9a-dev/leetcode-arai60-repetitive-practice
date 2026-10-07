use std::collections::{HashMap, HashSet};

struct NumberOfConnectedComponentsInAnUndirectedGraph {}
impl NumberOfConnectedComponentsInAnUndirectedGraph {
    /*
     * 問題の理解:
     * 0-n-1個のラベルが付いたn個のノードを持つ無向グラフを考える。
     * 整数nと配列edgesが与えられ、edges[i] = [ai,bi]
     * はグラフ内でaiとabの間にエッジがあることを表している。
     * グラフ内の連結成分の数を返す。
     *  連結成分とはエッジでつながっているグループを1つとしたときのグループの数。
     *  全てのノードがエッジで繋がっていれば、連結成分は１となる。
     *
     * memo:
     * edges[i]から見ていって、エッジのつながりをどこまで辿れるかということを考える。
     * ai < bi となるような制約が無い。
     * edges[i].len() == 2なのでifでai < bi となるように並べ替えながら走査する。
     * bi == ai+1 となるときはカウントをインクリメントしないという繰り返しで解けそう。
     * 時間計算量はedges.len()
     * Wrong Answerとなった。 edges=[[0,1],[1,2],[0,2],[3,4]]
     * [0,1]と[0,2]に対応できていない。
     * 隣接リストを作って解けそうな気がするが、具体的な作り方とコンポーネントの数え方が分からない。
     * 答えを見る。
     *
     * 解法の理解:
     * edgesから隣接リストを作る。ai: [bi], bi:
     * [ai]のように、あるノードをキーとしてルックアップしたときに繋がっているノードを全て取得できるようにする。
     * たどったノードをHashSetで管理する。
     * n =
     * 0のノードから初めて繋がっているノードを再帰的に見る。この時に見つけたノードは巡回済みとしてHashSetに記録する。
     * 巡回していないノードを起点として、全てのノードを巡回する時に何回巡回する必要があるかを数える。
     */

    pub fn count_components(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        let mut node_to_adjacency_nodes: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            let (node1, node2) = (edge[0], edge[1]);
            node_to_adjacency_nodes
                .entry(node1)
                .and_modify(|adjacency_nodes| adjacency_nodes.push(node2))
                .or_insert(vec![node2]);
            node_to_adjacency_nodes
                .entry(node2)
                .and_modify(|adjacency_nodes| adjacency_nodes.push(node1))
                .or_insert(vec![node1]);
        }

        let mut component_counts = 0;
        let mut visited_nodes: HashSet<i32> = HashSet::new();
        for node in 0..n {
            if visited_nodes.contains(&node) {
                continue;
            }

            Self::explore_adjacency_nodes(node, &node_to_adjacency_nodes, &mut visited_nodes);
            component_counts += 1;
        }

        component_counts
    }

    fn explore_adjacency_nodes(
        node: i32,
        node_to_adjacency_nodes: &HashMap<i32, Vec<i32>>,
        visited_nodes: &mut HashSet<i32>,
    ) {
        if !visited_nodes.insert(node) {
            return;
        }

        let Some(adjacency_nodes) = node_to_adjacency_nodes.get(&node) else {
            return;
        };
        for adjacency_node in adjacency_nodes {
            Self::explore_adjacency_nodes(*adjacency_node, node_to_adjacency_nodes, visited_nodes);
        }
    }

    /***** WRONG ANSWER *****/
    pub fn wrong_answer_count_components(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        let mut component_count = 1;
        let mut previous_right_node = None;
        for edge in edges {
            let (mut left_node, mut right_node) = (edge[0], edge[1]);
            if right_node < left_node {
                std::mem::swap(&mut left_node, &mut right_node);
            }

            if previous_right_node
                .is_some_and(|previous_right_node| previous_right_node != left_node)
            {
                component_count += 1;
            }
            previous_right_node = Some(right_node);
        }

        component_count
    }
}
