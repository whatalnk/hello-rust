use proconio::input;

fn main() {
    input! {
        mut k: i64,
    }
    let mut v = vec![];
    let mut p: i64 = 2;
    while p * p <= k {
        let mut e: i64 = 0;
        while k % p == 0 {
            k /= p;
            e += 1;
        }
        if e > 0 {
            v.push((p, e));
        }
        p += 1;
    }
    if k != 1 {
        v.push((k, 1));
    }
    let mut ans: i64 = 0;
    for (p, e) in &v {
        ans = ans.max(p * e);
    }
    println!("{}", ans);
}
