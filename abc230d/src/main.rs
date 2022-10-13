use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        lr: [(usize, usize); n]
    }
    println!("{} {}", n, d);
    for i in 0..n {
        let (l, r) = lr[i];
        println!("{} {}", l, r);
    }
}
