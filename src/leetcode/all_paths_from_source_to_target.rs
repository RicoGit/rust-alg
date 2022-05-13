//! 797. All Paths From Source to Target

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::dfs(&graph, 0, &mut result, vec![]);
        result
    }

    fn dfs(graph: &Vec<Vec<i32>>, idx: usize, result: &mut Vec<Vec<i32>>, mut path: Vec<i32>) {
        path.push(idx as i32);

        if idx == graph.len() - 1 {
            result.push(path); // we reach last node
            return
        }

        for edge in graph[idx].clone() {
            Self::dfs(&graph, edge as usize, result, path.clone())
        }
    }
}

struct Solution;
