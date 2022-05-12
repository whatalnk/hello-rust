use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut n: Chars,
    }
    n.sort();
    n.reverse();
    let l = n.len();
    let mut ans = 0;
    for i in 0..(1 << l) {
        let mut left = 0;
        let mut right = 0;
        for j in 0..l {
            if (i & (1 << j)) > 0 {
                left = left * 10 + n[j].to_digit(10).unwrap();
            } else {
                right = right * 10 + n[j].to_digit(10).unwrap();
            }
            ans = ans.max(left * right);
        }
    }
    println!("{}", ans);
}
