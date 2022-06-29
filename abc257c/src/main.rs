use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        mut w: [i64; n]
    }
    let mut ws = vec![];
    let mut adult = 0;
    for i in 0..n {
        let si = s[i].to_digit(10).unwrap() as i64;
        adult += si;
        ws.push((w[i], si));
    }
    ws.sort();
    let mut ans = adult;
    let mut ans_ = ans;
    // All adult
    for i in 0..n {
        // i is child
        if ws[i].1 == 0 {
            ans_ += 1;
        } else {
            ans_ -= 1;
        }
        ans = ans.max(ans_);
    }
    // wi が同じものがあると、これではダメ
    println!("{}", ans);
}
