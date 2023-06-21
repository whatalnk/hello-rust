use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut x: i64,
        s: Chars
    }
    let mut xb = vec![];
    while x > 0 {
        if x % 2 == 0 {
            xb.push('0');
        } else {
            xb.push('1');
        }
        x /= 2;
    }
    xb.reverse();
    for si in s.iter().take(n) {
        if si == &'U' {
            xb.pop();
        } else if si == &'L' {
            xb.push('0');
        } else {
            // R
            xb.push('1');
        }
    }
    xb.reverse();
    let mut ans: i64 = 0;
    let mut a: i64 = 1;
    for xbi in &xb {
        if xbi == &'1' {
            ans += a;
        }
        a *= 2;
    }
    println!("{}", ans);
}
