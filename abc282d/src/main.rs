use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    println!("{} {}", n, m);
    println!("{:?}", uv);
}