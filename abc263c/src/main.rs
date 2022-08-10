use proconio::input;

fn f(n: usize, m: usize, cnt: usize, cur: usize, s: String) {
    if cnt < n {
        for i in (cur + 1)..=m {
            let s_ = "".to_string() + &s + &" ".to_string() + &i.to_string();
            f(n, m, cnt + 1, i, s_);
        }
    } else {
        println!("{}", s);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    for i in 1..=m {
        f(n, m, 1, i, i.to_string());
    }
}
