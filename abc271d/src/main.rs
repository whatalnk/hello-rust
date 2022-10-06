use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=s {
            if dp[i][j] {
                if j + ab[i].0 <= s {
                    dp[i + 1][j + ab[i].0] = true;
                }
                if j + ab[i].1 <= s {
                    dp[i + 1][j + ab[i].1] = true;
                }
            }
        }
    }
    if dp[n][s] {
        println!("Yes");
        let mut t = vec!['H'; n];
        for i in 0..n {
            let ii = n - 1 - i;
            if s >= ab[ii].0 && dp[ii][s - ab[ii].0] {
                t[ii] = 'H';
                s -= ab[ii].0;
            } else {
                t[ii] = 'T';
                s -= ab[ii].1;
            }
        }
        println!("{}", t.into_iter().collect::<String>());
    } else {
        println!("No");
    }
}
