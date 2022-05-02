//! 399. Evaluate Division

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>
    ) -> Vec<f64> {
        let mut graph: HashMap<&str, HashMap<&str, f64>> = HashMap::new();

        for (idx, eq) in equations.iter().enumerate() {
            graph.entry(&eq[0])
                .or_insert_with(HashMap::new)
                .insert(&eq[1], values[idx]);

            graph.entry(&eq[1])
                .or_insert_with(HashMap::new)
                .insert(&eq[0], 1.0 / values[idx]);
        }

        println!("{:?}", graph);

        let mut result = vec![];
        for query in queries {
            let mut visited = HashSet::new();
            let answer = Self::dfs(&graph, query[0].as_str(), query[1].as_str(), &mut visited);
            result.push(answer)
        }

        result
    }

    fn dfs(graph: &HashMap<&str, HashMap<&str, f64>>, next: &str, target: &str, visited: &mut HashSet<String>) -> f64 {
        visited.insert(next.to_string());
        if let Some(nodes) = graph.get(next) {
                if let Some(value) = nodes.get(target) {
                    return *value
                } else {
                    for node in nodes {
                        if !visited.contains(&node.0.to_string()) {
                            let res = Self::dfs(&graph, node.0, target, visited);
                            if res != -1.0 {
                                return res * node.1
                            }
                        }
                    }
                }
            }
        -1.0
    }
}

struct Solution;
