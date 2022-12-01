use proconio::input;

fn f(a: f64, b: i64, n: i64) -> f64 {
    return (b * n) as f64 + a / (n as f64 + 1.0).sqrt();
}
fn main() {
    input! {
        a: f64,
        b: i64
    }
    let mut l = 0;
    let mut r = a as i64 / b;
    while r - l > 2 {
        let m1 = (l * 2 + r) / 3;
        let m2 = (l + r * 2) / 3;
        if f(a, b, m1) > f(a, b, m2) {
            l = m1;
        } else {
            r = m2;
        }
    }
    let mut ans = a;
    for i in l..=r {
        ans = ans.min(f(a, b, i));
    }
    println!("{}", ans);
}
