//! 844. Backspace String Compare

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_arr = s.into_bytes();
        let t_arr = t.into_bytes();

        let mut s_idx = Some(s_arr.len() - 1);
        let mut t_idx = Some(t_arr.len() - 1);

        loop {
            skip_removed(&mut s_idx, &s_arr);
            skip_removed(&mut t_idx, &t_arr);

            if s_idx.is_none() && t_idx.is_none() {
                return true;
            } else if s_idx.is_none() || t_idx.is_none() {
                return false;
            }

            if s_arr[s_idx.unwrap()] != t_arr[t_idx.unwrap()] {
                // println!("{} != {}", s_arr[s_idx.unwrap()] as char, t_arr[t_idx.unwrap()] as char);
                return false;
            }

            dec(&mut s_idx);
            dec(&mut t_idx);
        }
    }
}

fn skip_removed(idx: &mut Option<usize>, arr: &[u8]) {
    let mut skip = 0;
    if idx.is_none() || arr[idx.unwrap()] != b'#' {
        return;
    }

    while idx.is_some() {
        if arr[idx.unwrap()] == b'#' {
            skip += 1;
            dec(idx);
        } else if skip > 0 {
            skip -= 1;
            dec(idx)
        } else {
            break;
        }
    }
}

fn dec(idx: &mut Option<usize>) {
    *idx = idx.and_then(|i| i.checked_sub(1));
}

struct Solution;
