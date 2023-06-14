use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    }
    let mut t = 0.0;
    let mut ans = 0.0;
    for abi in ab.iter().take(n) {
        t += abi.0 / abi.1;
    }
    t /= 2.0;
    for abi in ab.iter().take(n) {
        ans += (abi.0).min(t * abi.1);
        t -= (abi.0 / abi.1).min(t);
    }
    println!("{}", ans);
}
