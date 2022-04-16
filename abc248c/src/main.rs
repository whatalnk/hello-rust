use proconio::input;

fn main() {
    let md = 998244353;
    input! {
        n: usize,
        m: usize,
        k: usize
    }
    let mut dp: Vec<Vec<u128>> = vec![vec![0; n]; m];
    for r in 0..m {
        if r <= k {
            dp[r][0] = 1;
        }
    }
    for c in 0..(n - 1) {
        for r in 0..m {
            for d in 0..m {
                if r + d + 2 <= k {
                    dp[d][c + 1] += dp[r][c];
                    dp[d][c + 1] %= md;
                }
            }
        }
    }
    let mut ans = 0;
    for r in 0..m {
        ans += dp[r][n - 1];
        ans %= md;
    }
    println!("{}", ans);
}
