use proconio::input;

fn main() {
    let md = 998244353;
    input! {
        n: usize,
        m: usize,
        k: usize
    }
    let mut dp: Vec<Vec<u128>> = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..k {
            for d in 1..=m {
                if j + d <= k {
                    dp[i + 1][j + d] += dp[i][j];
                    dp[i][j] %= md;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 1..=k {
        ans += dp[n][i];
        ans %= md;
    }
    println!("{}", ans);
}
