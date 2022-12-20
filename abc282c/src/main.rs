use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut quotes = vec![];
    for i in 0..n {
        if s[i] == '"' {
            quotes.push(i);
        }
    }
    let mut l = 0;
    let len = quotes.len();
    let mut ans = vec![];
    for i in 0..len {
        let ss = &s[l..quotes[i]].iter().collect::<String>();
        if i % 2 == 0 {
            ans.push(ss.replace(",", ".").clone());
        } else {
            ans.push(ss.clone())
        }
        l = quotes[i];
    }
    let ss = &s[l..n].iter().collect::<String>().replace(",", ".");
    ans.push(ss.replace(",", ".").clone());
    println!("{}", ans.join(""));
}
