use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i64,
        ab: [(i64, i64); n],
    }
    println!("{} {}", n, s);
    println!("{:?}", ab);
}
