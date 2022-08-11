use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut ans: i64 = 0;
    let md: i64 = 998244353;
    for i in 1..=n {
        let mut dp = vec![vec![vec![0; i]; i + 1]; n + 1];
        dp[0][0][0] = 1;
        for j in 0..n {
            for k in 0..=i {
                for l in 0..i {
                    dp[j + 1][k][l] += dp[j][k][l];
                    dp[j + 1][k][l] %= md;
                    if k != i {
                        dp[j + 1][k + 1][(l + a[j] as usize) % i] += dp[j][k][l];
                        dp[j + 1][k + 1][(l + a[j] as usize) % i] %= md;
                    }
                }
            }
        }
        ans += dp[n][i][0];
        ans %= md;
    }
    println!("{}", ans % md);
}
