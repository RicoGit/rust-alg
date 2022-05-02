//! 1202. Smallest String With Swaps

//! 1202. Smallest String With Swaps


use std::collections::{HashSet, HashMap};
use std::str::Bytes;


impl Solution {

    pub fn smallest_string_with_swaps(mut s: String, mut pairs: Vec<Vec<i32>>) -> String {

        let mut graph = vec![vec![]; s.len()];
        for pair in pairs {
            let from = pair[0] as usize;
            let to = pair[1] as usize;
            graph[from].push(to);
            graph[to].push(from);
        }

        let mut visited = vec![false; s.len()];

        for idx in 0..s.len() {
            if  visited[idx] { continue };

            let mut indexes = vec![];
            Self::subtree(idx, &graph, &mut visited, &mut indexes);
            s = Self::sort(&mut s, indexes);
        }

        s
    }

    fn sort(s: &mut String, mut indexes: Vec<usize>) -> String {
        let mut bytes = s.bytes().collect::<Vec<u8>>();
        let mut selected = indexes.iter().map(|idx| bytes[*idx]).collect::<Vec<u8>>();
        selected.sort();
        selected.reverse();
        indexes.sort();

        for idx in indexes {
            bytes[idx] = selected.pop().unwrap()
        }

        String::from_utf8_lossy(&bytes).to_string()
    }

    fn subtree(idx: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, indexes: &mut Vec<usize>) {
        visited[idx] = true;
        indexes.push(idx);
        if let Some(children) = graph.get(idx) {
            for &child in children {
                if !visited[child] {
                    Self::subtree(child, graph, visited, indexes);
                }
            }
        }
    }
}


struct Solution;

