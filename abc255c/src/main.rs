use proconio::input;

fn main() {
    input! {
        x: i128,
        a: i128,
        d: i128,
        n: i128
    }
    let mut ans = 0;
    let s0 = a;
    let sn = a + (n - 1) * d;
    if d > 0 {
        if x < s0 {
            ans = s0 - x;
        } else if x < sn {
            ans = (x - a) % d;
        } else {
            ans = x - sn;
        }
    } else {
        if x < sn {
            ans = sn - x;
        } else if x < sn {
            ans = (x - a) % d;
        } else {
            ans = x - s0;
        }
    }
    println!("{}", ans);
}
