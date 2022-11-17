use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
    }
    for i in 0..n {
        let x = a[i];
        println!("{} => ({} {})", x, x, (x + 1) % m);
    }
}
