use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        l: usize,
        r: usize,
        mut s: Chars
    }
    s[(l - 1)..r].reverse();
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
