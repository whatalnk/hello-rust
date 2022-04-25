use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i32, i32); n]
    }
    println!("{}", n);
    for i in 0..n {
        println!("{} {}", ab[i].0, ab[i].1);
    }
}
