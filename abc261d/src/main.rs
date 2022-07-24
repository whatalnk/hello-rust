use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [i64; n],
        cy: [(usize, i64); m]
    }
    x.insert(0, 0);
    let mut y = vec![0; n + 1];
    for i in 0..m {
        let (c, y_) = cy[i];
        y[c] = y_;
    }
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=i {
            dp[i][j] = dp[i - 1][j - 1] + x[i] + y[j];
        }
        dp[i][0] = 0;
        for j in 0..i {
            dp[i][0] = dp[i][0].max(dp[i - 1][j]);
        }
    }
    let mut ans = 0;
    for i in 0..=n {
        ans = ans.max(dp[n][i]);
    }
    println!("{}", ans);
}
