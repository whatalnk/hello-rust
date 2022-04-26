use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    }
    let mut t = 0.0;
    let mut ans = 0.0;
    for i in 0..n {
        t += ab[i].0 / ab[i].1;
    }
    t /= 2.0;
    for i in 0..n {
        ans += (ab[i].0).min(t * ab[i].1);
        t -= (ab[i].0 / ab[i].1).min(t);
    }
    println!("{}", ans);
}
