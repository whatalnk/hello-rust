use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize, usize); q]
    }
    println!("{:?}", s);
    println!("{}", q);
    println!("{:?}", tk);
}
