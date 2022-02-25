use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    if s[n - 2] == 'e' && s[n - 1] == 'r' {
        println!("er");
    }
    if n >= 3 && s[n - 3] == 'i' && s[n - 2] == 's' && s[n - 1] == 't' {
        println!("ist");
    }
}
