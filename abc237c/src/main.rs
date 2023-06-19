use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let a: String = s.iter().collect();
    let b: String = s.iter().rev().collect();
    if a == b {
        println!("Yes");
        return;
    }
    let mut ans = false;
    let n = s.len();
    let mut left = 0;
    for si in s.iter().take(n) {
        if si == &'a' {
            left += 1;
        } else {
            break;
        }
    }
    let mut right = 0;
    for i in 0..n {
        if s[n - 1 - i] == 'a' {
            right += 1;
        } else {
            break;
        }
    }
    if left <= right {
        let a: String = s[left..(n - right)].iter().collect();
        let b: String = s[left..(n - right)].iter().rev().collect();
        if a == b {
            ans = true;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
