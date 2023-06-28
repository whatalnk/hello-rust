use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m]
    }
    let mut g = vec![vec![false; n]; n];
    for uvi in uv.iter().take(m) {
        let (u, v) = uvi;
        g[u - 1][v - 1] = true;
        g[v - 1][u - 1] = true;
    }
    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if g[i][j] && g[j][k] && g[k][i] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
