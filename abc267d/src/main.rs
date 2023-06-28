use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let minimum = -1_000_000_000_000_000_000;
    let mut dp = vec![vec![minimum; 2001]; 2001];
    dp[0][0] = 0i64;
    for i in 1..=n {
        for j in 0..=n {
            if j == 0 {
                dp[i][0] = 0;
            } else if j > i {
                dp[i][j] = minimum;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - 1] + a[i - 1] * (j as i64));
            }
        }
    }
    println!("{}", dp[n][m]);
}
