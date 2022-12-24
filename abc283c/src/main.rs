use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut ans = 0;
    let mut skip = false;
    for i in 0..(n - 1) {
        if skip {
            skip = false;
            continue;
        }
        if s[i] == '0' && s[i + 1] == '0' {
            ans += 1;
            skip = true;
        } else {
            ans += 1;
            skip = false;
        }
    }
    if skip {
        println!("{}", ans);
    } else {
        println!("{}", ans + 1);
    }
}
