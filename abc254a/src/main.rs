use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars
    }
    println!("{}{}", n[1], n[2]);
}
