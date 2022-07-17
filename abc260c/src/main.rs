use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64
    }
    let mut g = vec![vec![0; 11]; 2];
    g[0][n] = 1;
    for i in 1..n {
        // 1
        g[0][n - i] += g[0][n - i + 1];
        g[1][n - i + 1] += g[0][n - i + 1] * x;
        // 2
        g[0][n - i] += g[1][n - i + 1];
        g[1][n - i] += g[1][n - i + 1] * y;
    }
    println!("{}", g[1][1]);
}
