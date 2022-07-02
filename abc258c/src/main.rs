use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
        query: [(usize, usize); q]
    }
    let mut st = 1;
    let mut ed = n;
    for i in 0..q {
        let (t, x) = query[i];
        if t == 1 {
            if ed >= x {
                st = ed - x + 1;
            } else {
                st = n + ed - x + 1;
            }
            ed = st + n - 1;
            if ed >= n {
                ed -= n;
            }
        } else {
            let mut xx = st + x - 1;
            if xx > n {
                xx -= n;
            }
            println!("{}", s[xx - 1]);
        }
    }
}
