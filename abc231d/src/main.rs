use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    println!("{} {}", n, m);
    println!("{:?}", ab);
}
