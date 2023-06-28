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
    for cyi in cy.iter().take(m) {
        let (c, y_) = cyi;
        y[*c] = *y_;
    }
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for (i, xi) in x.iter().enumerate().take(n + 1).skip(1) {
        for (j, yj) in y.iter().enumerate().take(i + 1).skip(1) {
            dp[i][j] = dp[i - 1][j - 1] + xi + yj;
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
