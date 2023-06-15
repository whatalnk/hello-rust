use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut t = "oxx".to_string();
    for _ in 0..20 {
        t += "oxx";
    }
    let tt = t.as_str();
    let n = s.len();
    if s == tt[0..n] || s == tt[1..n + 1] || s == tt[2..n + 2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
