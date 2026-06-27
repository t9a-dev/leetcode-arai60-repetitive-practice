use core::range::RangeInclusive;
use std::collections::{HashMap, HashSet, VecDeque};

struct NumberOfConnectedComponentsInAnUndirectedGraph {}
impl NumberOfConnectedComponentsInAnUndirectedGraph {
    /*
    LeetCode代替:
    https://neetcode.io/problems/count-connected-components/question

    問題の理解:
    n個のノードからなるグラフが与えられる。
    整数nと配列edgesが与えられ、edges[i] = [a, b]はグラフ内でaとbの間にエッジが存在することを示している。
    グラフ内の連結成分の数を返す。
    入力の制約
    - a < b
    - a != b
    - 重複する辺は存在しない

    memo:
    連結成分とはつながっているノードのグループのこと。0-1-2 3-4 であればグループが2つなので答えは2になる。
    手が止まったので答えを写経する。

    解法の理解:
    隣接リストをHashMapで作っている。edges[i] = [a, b] ノードa,ノードbが含まれている。ノードa,bはつながっていることを表している。
    あるノードとつながっている（隣接）ノードを管理するために、ノードをキー、隣接しているノードを値としてHashMapで管理している。
    このHashMapを利用すると、あるノードに隣接しているノードをすべて取得できる。
    さらに取得したノードをキーとして隣接してるノード取得して...と隣接しているノードを再帰的に辿ることができる。
    一度見たノードをHashSetで管理（メモ化）しておいて、早期リターンすることで探索済みかを高速に判定できる。

    */
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut node_to_adjacency_nodes: HashMap<i32, Vec<_>> = HashMap::new();
        for edge in edges {
            let Ok([node1, node2]): Result<[i32; 2], _> = edge.try_into() else {
                continue;
            };
            node_to_adjacency_nodes
                .entry(node1)
                .and_modify(|nodes| nodes.push(node2))
                .or_insert(vec![node2]);
            node_to_adjacency_nodes
                .entry(node2)
                .and_modify(|nodes| nodes.push(node1))
                .or_insert(vec![node1]);
        }

        let mut components_count = 0;
        let mut visited_nodes: HashSet<_> = HashSet::new();
        for node in 0..n {
            if visited_nodes.contains(&node) {
                continue;
            }

            Self::explore_component(&node_to_adjacency_nodes, node, &mut visited_nodes);
            components_count += 1;
        }

        components_count
    }

    fn explore_component(
        node_to_adjacency_nodes: &HashMap<i32, Vec<i32>>,
        node: i32,
        visited_nodes: &mut HashSet<i32>,
    ) {
        if !visited_nodes.insert(node) {
            return;
        }

        let Some(nodes) = node_to_adjacency_nodes.get(&node) else {
            return;
        };
        for node in nodes {
            Self::explore_component(node_to_adjacency_nodes, *node, visited_nodes);
        }
    }
}

struct NumberOfConnectedComponentsInAnUndirectedGraph2 {}
impl NumberOfConnectedComponentsInAnUndirectedGraph2 {
    // iterative(反復)な実装ができなかったのでLLMにきいて写経
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut node_to_adjacency_nodes: HashMap<_, Vec<_>> = HashMap::new();
        for edge in edges {
            let Ok([node1, node2]): Result<[i32; 2], _> = edge.try_into() else {
                continue;
            };
            node_to_adjacency_nodes
                .entry(node1)
                .or_default()
                .push(node2);
            node_to_adjacency_nodes
                .entry(node2)
                .or_default()
                .push(node1);
        }

        let mut components_count = 0;
        let mut visited_nodes: HashSet<_> = HashSet::new();
        for node in 0..n {
            if visited_nodes.contains(&node) {
                continue;
            }

            components_count += 1;
            let mut stack = vec![node];
            while let Some(node) = stack.pop() {
                let Some(adjacency_nodes) = node_to_adjacency_nodes.get(&node) else {
                    continue;
                };
                for adjacency_node in adjacency_nodes {
                    if !visited_nodes.insert(adjacency_node) {
                        continue;
                    }
                    stack.push(*adjacency_node);
                }
            }
        }

        components_count
    }
}
