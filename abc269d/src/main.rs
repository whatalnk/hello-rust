use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }
    println!("{}", n);
    for i in 0..n {
        println!("{} {}", xy[i].0, xy[i].1);
    }
}
