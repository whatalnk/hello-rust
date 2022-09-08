use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    println!("{} {}", n, m);
    println!("{:?}", a);
}
