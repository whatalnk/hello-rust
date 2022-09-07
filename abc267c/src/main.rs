use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }
    let mut s = 0;
    for i in 0..m {
        s += (i as i64 + 1) * a[i];
    }
    let mut ans = s;
    for i in 1..(n - m + 1) {
        let d1 = b[i - 1 + m] - b[i - 1];
        let d2 = (m as i64) * a[i - 1 + m];
        s = s - d1 + d2;
        ans = ans.max(s);
    }
    println!("{}", ans);
}
