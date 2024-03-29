use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [(usize, i64, i64); n]
    }
    let mut cc = vec![];
    for item in c.iter() {
        if item.0 == 1 {
            cc.push((item.1 * 10, item.2 * 10));
        } else if item.0 == 2 {
            cc.push((item.1 * 10, item.2 * 10 - 1));
        } else if item.0 == 3 {
            cc.push((item.1 * 10 + 1, item.2 * 10));
        } else {
            cc.push((item.1 * 10 + 1, item.2 * 10 - 1));
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if !((cc[i].1 < cc[j].0) || (cc[i].0 > cc[j].1)) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
