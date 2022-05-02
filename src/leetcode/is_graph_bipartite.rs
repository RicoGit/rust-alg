//! 785. Is Graph Bipartite?

#[derive(Clone, PartialEq, Eq, Debug)]
enum Color {
    Red,
    Black,
    Unknown,
}
impl Color {
    fn next(&self) -> Self {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
            Color::Unknown => Color::Unknown,
        }
    }
}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![Color::Unknown; graph.len()];
        let mut stack = vec![(0, Color::Red)];

        for current_idx in 0..graph.len() {
            while let Some((idx, color)) = stack.pop() {
                match &visited[idx] {
                    Color::Unknown => {
                        let next_color = color.next();
                        visited[idx] = color;
                        for node_idx in &graph[idx] {
                            stack.push((*node_idx as usize, next_color.clone()))
                        }
                    }
                    other => {
                        if *other == color {
                            continue;
                        }
                        if *other != color {
                            return false;
                        }
                    }
                }
            }

            if visited[current_idx] == Color::Unknown {
                stack.push((current_idx, Color::Black))
            }
        }
        // println!("{:?}", visited);

        true
    }
}




struct Solution;
