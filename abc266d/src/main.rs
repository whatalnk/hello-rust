use proconio::input;

fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, i64); n],
    }
    let t_max = 100_001;
    let minimum = -1_000_000_000_000_000_000i64;
    let mut xx = vec![0; t_max];
    let mut aa = vec![0; t_max];
    for txai in txa.iter().take(n) {
        let (t, x, a) = txai;
        xx[*t] = *x;
        aa[*t] = *a;
    }
    let mut g = vec![vec![minimum; t_max]; 5];
    g[0][0] = 0;
    for t in 1..t_max {
        for i in 0..5 {
            g[i][t] = g[i][t - 1];
            if i != 0 {
                g[i][t] = g[i][t].max(g[i - 1][t - 1]);
            }
            if i != 4 {
                g[i][t] = g[i][t].max(g[i + 1][t - 1]);
            }
        }
        g[xx[t]][t] += aa[t];
    }
    let mut ans = 0;
    for gi in g.iter().take(5) {
        ans = ans.max(gi[t_max - 1]);
    }
    println!("{}", ans);
}
