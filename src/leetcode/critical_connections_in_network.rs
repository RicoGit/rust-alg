//! 1192. Critical Connections in a Network

use std::cmp::min;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in connections {
            let (i, j) = (edge[0], edge[1]);
            graph[i as usize].push(j as usize);
            graph[j as usize].push(i as usize);
        }
        let mut result = vec![];
        Self::dfs(0, 0, 0, &mut graph, &mut vec![-1; n], &mut vec![-1; n], &mut result);
        result
    }

    fn dfs(
        idx1: usize,
        idx2: usize,
        count: i32,
        graph: &Vec<Vec<usize>>,
        pre: &mut Vec<i32>,
        low: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        let count = count + 1;
        pre[idx1] = count;
        low[idx1] = count;

        for &node in &graph[idx1] {
            if pre[node] == -1 {
                Self::dfs(node, idx1, count, graph, pre, low, result);
                if low[node] == pre[node] {
                    result.push(vec![idx1 as i32, idx2 as i32])
                }
            }
            if node != idx2 {
                low[idx1] = min(low[node], low[idx1])
            }
        }
    }
}

 struct Solution;
