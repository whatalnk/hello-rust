use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: Chars
    }
    println!("{} {} {:?}", n, x, s);
}
