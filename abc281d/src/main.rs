use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: i64,
        a: [i64; n],
    }
    let mut dp = vec![vec![vec![-1; d as usize]; k + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..k + 1 {
            for kk in 0..(d as usize) {
                if dp[i][j][kk] == -1 {
                    continue;
                }
                dp[i + 1][j][kk] = dp[i + 1][j][kk].max(dp[i][j][kk]);
                if j != k {
                    dp[i + 1][j + 1][((kk as i64 + a[i]) % d) as usize] = dp[i + 1][j + 1]
                        [((kk as i64 + a[i]) % d) as usize]
                        .max(dp[i][j][kk] + a[i]);
                }
            }
        }
    }
    println!("{}", dp[n][k][0]);
}
