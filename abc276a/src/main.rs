use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut ans = -1;
    let n = s.len();
    for (i, si) in s.iter().enumerate().take(n) {
        if si == &'a' {
            ans = (i + 1) as isize;
        }
    }
    println!("{}", ans);
}
