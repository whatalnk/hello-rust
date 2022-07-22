use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        x: i128,
        s: Chars
    }
    let mut xx = x;
    let mut k: i128 = 0;
    while xx > 1 {
        xx /= 2;
        k += 1;
    }
    let mut r: i128 = 1;
    for _ in 0..k {
        r *= 2;
    }
    r = x - r;
    for i in 0..n {
        if s[i] == 'U' {
            k -= 1;
            r /= 2;
        } else if s[i] == 'L' {
            k += 1;
            r *= 2;
        } else {
            // 'R'
            k += 1;
            r = r * 2 + 1;
        }
    }
    let mut ans = 1;
    for _ in 0..k {
        ans *= 2;
    }
    ans += r;
    println!("{}", ans);
}
