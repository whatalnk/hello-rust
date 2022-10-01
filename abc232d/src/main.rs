use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut f = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            let ii = h - i - 1;
            let jj = w - j - 1;
            if c[ii][jj] != '#' {
                f[ii][jj] = f[ii + 1][jj].max(f[ii][jj + 1]) + 1;
            }
        }
    }
    println!("{}", f[0][0]);
}
