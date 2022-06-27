use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [i64; n]
    }
    println!("{}", n);
    println!("{:?}", s);
    println!("{:?}", w);
}
