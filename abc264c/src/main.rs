use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[i64; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[i64; w2]; h2]
    }
    let ith = (0..h1).combinations(h2).collect::<Vec<_>>();
    let itw = (0..w1).combinations(w2).collect::<Vec<_>>();
    for ih in 0..ith.len() {
        for iw in 0..itw.len() {
            let mut ans = true;
            'outer: for i in 0..h2 {
                for j in 0..w2 {
                    if a[ith[ih][i]][itw[iw][j]] != b[i][j] {
                        ans = false;
                        break 'outer;
                    }
                }
            }
            if ans {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
