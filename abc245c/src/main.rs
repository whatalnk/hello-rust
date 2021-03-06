use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n]
    }
    let mut dp = vec![false; n];
    let mut ep = vec![false; n];
    dp[0] = true;
    ep[0] = true;
    for i in 1..n {
        if dp[i - 1] {
            if (a[i - 1] - a[i]).abs() <= k {
                dp[i] = true;
            }
            if (a[i - 1] - b[i]).abs() <= k {
                ep[i] = true;
            }
        }
        if ep[i - 1] {
            if (b[i - 1] - a[i]).abs() <= k {
                dp[i] = true;
            }
            if (b[i - 1] - b[i]).abs() <= k {
                ep[i] = true;
            }
        }
    }
    if dp[n - 1] || ep[n - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
