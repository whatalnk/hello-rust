use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: String
    }
    let ss: String = s.iter().collect();
    if ss == t {
        println!("Yes");
        return;
    }
    let n = s.len();
    for i in 0..n - 1 {
        s.swap(i, i + 1);
        let ss: String = s.iter().collect();
        if ss == t {
            println!("Yes");
            return;
        }
        s.swap(i, i + 1);
    }
    println!("No");
}
