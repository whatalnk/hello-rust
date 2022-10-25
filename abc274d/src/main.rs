use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        a: [i64; n]
    }
    println!("{} {} {}", n, x, y);
    println!("{:?}", a);
}
