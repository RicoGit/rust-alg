//! 841. Keys and Rooms

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        visited[0] = true;
        let mut stack: Vec<i32> = vec![];
        stack.extend(rooms[0].iter()); // push keys from room 0

        while let Some(room) = stack.pop() {
            let room = room as usize;
            if !visited[room] {
                visited[room] = true;
                stack.extend(rooms[room].iter())
            }
        }

        visited.into_iter().filter(|r| *r).count() == rooms.len()
    }
}

struct Solution;
