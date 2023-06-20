use proconio::input;

fn main() {
    let xx = 998_244_353;
    input! {
        n: usize
    }
    let mut m: Vec<Vec<i64>> = vec![vec![0; n]; 9];
    for mr in m.iter_mut().take(9) {
        mr[0] = 1;
    }
    for c in 1..n {
        for r in 0..9 {
            if r == 0 {
                m[r][c] = m[r][c - 1] + m[r + 1][c - 1];
            } else if r == 8 {
                m[r][c] = m[r - 1][c - 1] + m[r][c - 1];
            } else {
                m[r][c] = m[r - 1][c - 1] + m[r][c - 1] + m[r + 1][c - 1];
            }
            m[r][c] %= xx;
        }
    }
    let mut ans = 0;
    for mr in m.iter().take(9) {
        ans += mr[n - 1];
        ans %= xx;
    }
    println!("{:?}", ans);
}
