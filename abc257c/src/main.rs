use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        mut w: [i64; n]
    }
    let mut ws = vec![];
    for i in 0..n {
        ws.push((w[i], s[i]));
    }
    ws.sort();
    // w.sort();
    let mut ss = vec![];
    ss.push(0);
    ss.push(ws[0].1.to_digit(10).unwrap() as i64);
    for i in 1..n {
        ss.push(ss[i] + ws[i].1.to_digit(10).unwrap() as i64);
    }
    ss.push(ss[n - 1]);
    let mut ans = 0;
    for i in 0..=n {
        let may_be_child = i as i64;
        let may_be_adult = n as i64 - may_be_child;
        let adult = ss[i];
        let child = n as i64 - adult;
        ans = ans.max(may_be_child.min(child) + may_be_adult.min(adult));
    }
    println!("{}", ans);
}
