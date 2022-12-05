use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    };
    let mut v = vec![];
    v.push(s[0].to_string());
    for i in 1..s.len() {
        v.push((s[i] - s[i - 1]).to_string());
    }
    println!("{}", v.join(" "));
}
