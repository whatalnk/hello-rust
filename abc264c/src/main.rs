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
    for ithi in &ith {
        for itwi in &itw {
            let mut ans = true;
            'outer: for (i, ithii) in ithi.iter().enumerate().take(h2) {
                for (j, itwij) in itwi.iter().enumerate().take(w2) {
                    if a[*ithii][*itwij] != b[i][j] {
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
