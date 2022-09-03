use proconio::input;

fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, i64); n],
    }
    let t_max = 100001;
    let mut tx = vec![vec![0; t_max]; 5];
    for i in 0..n {
        let (t, x, a) = txa[i];
        tx[x][t] = a;
    }
    let mut g = vec![vec![0; t_max]; 5];
    let mut mask = vec![vec![1; t_max]; 5];
    mask[2][1] = 0;
    mask[3][1] = 0;
    mask[4][1] = 0;
    mask[3][2] = 0;
    mask[4][2] = 0;
    mask[4][3] = 0;
    for j in 1..t_max {
        for i in 0..5 {
            if i == 0 {
                g[i][j] = (g[i][j - 1] * mask[i][j - 1]).max(g[i + 1][j - 1] * mask[i + 1][j - 1]);
            } else if i == 4 {
                g[i][j] = (g[i][j - 1] * mask[i][j - 1]).max(g[i - 1][j - 1] * mask[i - 1][j - 1]);
            } else {
                g[i][j] = (g[i - 1][j - 1] * mask[i - 1][j - 1])
                    .max(g[i][j - 1] * mask[i][j - 1])
                    .max(g[i + 1][j - 1] * mask[i + 1][j - 1]);
            }
            g[i][j] += tx[i][j] * mask[i][j];
        }
    }
    println!("{}", g[4][t_max - 1])
}
