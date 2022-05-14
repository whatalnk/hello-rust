use proconio::input;

fn main() {
    input! {
         s: String,
    }
    let mut ans = "".to_string();
    let mut n = 6;
    if s.len() == 3 {
        n = 2;
    } else if s.len() == 2 {
        n = 3;
    }
    for _ in 0..n {
        ans += &s;
    }
    println!("{}", ans);
}
