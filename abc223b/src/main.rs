use proconio::input;

fn main() {
    input! {
        mut s: String
    }
    let mut ans: Vec<String> = Vec::new();
    let n = s.len();
    for i in 0..s.len() {
        let x = format!("{}{}", &s[i..n], &s[0..i]);
        ans.push(x);
    }
    ans.sort();
    println!("{}", ans.first().unwrap());
    println!("{}", ans.last().unwrap());
}
