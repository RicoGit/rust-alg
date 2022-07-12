//! 299. Bulls and Cows

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let secret = secret.into_bytes();
        let guess = guess.into_bytes();

        let mut secret_freq = [0; 10];
        let mut guess_freq = [0; 10];

        let mut bulls = 0;
        let mut cows = 0;

        for idx in 0..guess.len() {
            let s_digit = secret[idx] - b'0';
            let g_digit = guess[idx] - b'0';
            if s_digit == g_digit  {
                bulls += 1;
            } else {
                secret_freq[s_digit as usize] += 1;
                guess_freq[g_digit as usize] += 1;
            }
        }

        for n in 0..10 {
            cows += secret_freq[n].min(guess_freq[n])
        }

        format!("{}A{}B", bulls, cows)
    }
}

struct Solution;