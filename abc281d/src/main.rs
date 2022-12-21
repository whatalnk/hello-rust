use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: i64,
        a: [i64; n],
    }
    println!("{} {} {}", n, k, d);
    println!("{:?}", a);
}
