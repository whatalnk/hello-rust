use proconio::input;

fn main() {
    input! {
        _n: i64,
        m: i64,
        x: i64,
        t: i64,
        d: i64
    }
    if m <= x {
        println!("{}", t - d * x + d * m);
    } else {
        println!("{}", t);
    }
}
