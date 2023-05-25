use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }
    let n = s.len();
    s.insert(0, ' ');
    let md = 1_000_000_007;
    let t: Vec<char> = " chokudai".chars().collect();
    let mut dp = vec![vec![0; 9]; n + 1];
    for i in 0..=n {
        for (j, tj) in t.iter().enumerate().take(8 + 1) {
            if j == 0 {
                dp[i][j] = 1;
            } else if i == 0 {
                dp[i][j] = 0;
            } else if s[i] != *tj {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
            }
            dp[i][j] %= md;
        }
    }
    println!("{}", dp[n][8]);
}
