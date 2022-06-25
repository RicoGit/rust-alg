//! 2299. Strong Password Checker II

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 { return false }
        // let specials = vec!['!','@','#','$','%','^','&','*','(',')','-','+'].into_iter().map(|c| c as u8).collect::<Vec<_>>();
        let specials = vec![33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 45, 43];
        let mut prev = 0;
        let mut bit_vector: usize = 0;
        for byte in password.bytes() {
            // repeats
            if prev == byte { return false };
            prev = byte;

            // special chars !@#$%^&*()-+ precalculated as `specials`
            if specials.contains(&byte) { bit_vector |= 1; continue }

            // numbers
            if byte > 47 && byte < 58 { bit_vector |= 2; continue }

            // uppercase
            if byte > 64 && byte < 91 { bit_vector |= 4; continue }

            // lowercase
            if byte > 96 && byte < 123 { bit_vector |= 8; continue }
        }

        bit_vector.count_ones() == 4
    }
}

struct Solution;