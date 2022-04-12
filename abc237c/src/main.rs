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
    for i in 0..n {
        if s[n - 1 - i] == 'a' {
            let a: String = s[0..(n - 1 - i)].iter().collect();
            let b: String = s[0..(n - 1 - i)].iter().rev().collect();
            if a == b {
                ans = true;
                break;
            }
        } else {
            break;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
