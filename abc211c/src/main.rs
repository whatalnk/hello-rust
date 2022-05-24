use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let t: Vec<char> = "chokudai".chars().collect();
    let mut dp = vec![vec![0; 9]; s.len() + 1];
    for i in 0..=s.len() {
        for j in 0..=8 {
            if j == 0 {
                dp[i][j] = 1;
            } else if i == 0 {
                dp[i][j] = 0;
            } else if s[i - 1] != t[j - 1] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
            }
        }
    }
    println!("{:?}", dp[s.len()][8]);
}
