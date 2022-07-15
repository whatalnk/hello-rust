use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n + 1],
        c: [i64; n + m + 1]
    }
    let mut aa = vec![vec![0; n + m + 1]; m];
    let mut d = 0;
    for i in 0..(n + 1) {
        for j in 0..(m + 1) {
            aa[i][d + j] = a[i];
        }
        d += 1;
    }
    let mut b = vec![0; m + 1];
    b[0] = c[0] / a[0];
    for i in 0..(n + m + 1) {
        for j in 0..m {
            if i >= j && i <= m + j {
                println!("{}: {}*{}", i, j, i - j)
            }
        }
    }
}
