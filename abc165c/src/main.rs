use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        q: usize,
        abcd: [(i64, i64, i64, i64); q]
    }
    println!("{} {} {}", n, m, q);
    for i in 0..q {
        println!("{:?}", abcd[i]);
    }
}
