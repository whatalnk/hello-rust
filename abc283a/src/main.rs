use proconio::input;

fn main() {
    input! {
        a: i64,
        b: usize,
    };
    let mut ans = 1;
    for _ in 0..b {
        ans *= a;
    }
    println!("{}", ans);
}
