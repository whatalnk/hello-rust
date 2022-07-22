use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        x: i64,
        s: Chars
    }
    let mut xx = x;
    let mut k = 0;
    while xx > 1 {
        xx /= 2;
        k += 1;
    }
    let mut r = x - (2 as i64).pow(k);
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
    println!("{}", (2 as i64).pow(k) + r);
}
