use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t : [Chars; n]
    }
    println!("{}", n);
    println!("{:?}", s);
    println!("{:?}", t);
}
