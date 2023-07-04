use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        p: [i64; n],
    }
    for (i, pi) in p.iter().enumerate().take(n) {
        if pi == &x {
            println!("{}", i + 1);
            return;
        }
    }
}
