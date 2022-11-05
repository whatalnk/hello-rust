use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut ans = -1;
    let n = s.len();
    for i in 0..n {
        if s[i] == 'a' {
            ans = (i + 1) as isize;
        }
    }
    println!("{}", ans);
}
