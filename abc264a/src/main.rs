use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize
    }
    let s = "atcoder".chars().collect::<Vec<char>>();
    let mut ans = vec![];
    for i in l..=r {
        ans.push(s[i - 1].to_string());
    }
    println!("{}", ans.join(""));
}
