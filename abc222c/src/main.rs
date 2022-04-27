use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2*n]
    }
    println!("{} {}", n, m);
    for i in 0..(2 * n) {
        println!("{:?}", a[i]);
    }
}
