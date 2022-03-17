use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }
    let mut ans: Vec<String> = Vec::new();
    for _ in 0..s.len() {
        let x = s.remove(0);
        s.push(x);
        ans.push(s.iter().collect::<String>());
    }
    ans.sort();
    println!("{}", ans.first().unwrap());
    println!("{}", ans.last().unwrap());
}
