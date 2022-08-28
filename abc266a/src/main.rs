use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let l = s.len();
    println!("{}", s[(l + 1) / 2 - 1]);
}
