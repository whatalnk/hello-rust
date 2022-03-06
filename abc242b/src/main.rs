use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }
    s.sort();
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
