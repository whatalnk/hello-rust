use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }
    let n = s.len();
    s.insert(0, ' ');
    let md = 1000000007;
    let t: Vec<char> = " chokudai".chars().collect();
    let mut dp = vec![vec![0; 9]; n + 1];
    for i in 0..=n {
        for j in 0..=8 {
            if j == 0 {
                dp[i][j] = 1;
            } else if i == 0 {
                dp[i][j] = 0;
            } else if s[i] != t[j] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
            }
            dp[i][j] %= md;
        }
    }
    println!("{}", dp[n][8]);
}
