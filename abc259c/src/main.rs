use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }
    let mut ss = vec![];
    let mut cur = s[0];
    let mut cnt = 0;
    for si in &s {
        if si == &cur {
            cnt += 1;
        } else {
            ss.push((cur, cnt));
            cur = *si;
            cnt = 1;
        }
    }
    ss.push((cur, cnt));

    let mut tt = vec![];
    let mut cur = t[0];
    let mut cnt = 0;
    for ti in &t {
        if ti == &cur {
            cnt += 1;
        } else {
            tt.push((cur, cnt));
            cur = *ti;
            cnt = 1;
        }
    }
    tt.push((cur, cnt));
    if ss.len() != tt.len() {
        println!("No");
        return;
    }
    for i in 0..ss.len() {
        if ss[i].0 != tt[i].0 {
            println!("No");
            return;
        }
        if ss[i].1 > tt[i].1 {
            println!("No");
            return;
        }
        if ss[i].1 < tt[i].1 && ss[i].1 == 1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
