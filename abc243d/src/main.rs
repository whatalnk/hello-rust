use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut x: i64,
        s: Chars
    }
    for i in 0..n {
        if s[i] == 'U' {
            if x % 2 == 0 {
                x /= 2;
            } else {
                x = (x - 1) / 2;
            }
        } else if s[i] == 'L' {
            x *= 2;
        } else {
            // R
            x = x * 2 + 1;
        }
    }
    println!("{}", x);
}
