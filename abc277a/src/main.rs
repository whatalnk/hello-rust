use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        p: [i64; n],
    }
    for i in 0..n {
        if p[i] == x {
            println!("{}", i + 1);
            return;
        }
    }
}
