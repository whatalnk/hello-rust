use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h]
    }
    let mut hh = vec![];
    let mut ww = vec![];
    for n in 0..=h {
        for i in (0..h).combinations(n) {
            hh.push(i);
        }
    }
    for n in 0..=w {
        for i in (0..w).combinations(n) {
            ww.push(i);
        }
    }
    let mut ans = 0;
    for i in &hh {
        for j in &ww {
            let mut m = vec![vec![true; w]; h];
            for ii in i {
                for jj in 0..w {
                    m[*ii][jj] = false;
                }
            }
            for jj in j {
                for row in m.iter_mut() {
                    row[*jj] = false;
                }
            }
            let mut cnt = 0;
            for ii in 0..h {
                for jj in 0..w {
                    if m[ii][jj] && c[ii][jj] == '#' {
                        cnt += 1;
                    }
                }
            }
            if cnt == k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
