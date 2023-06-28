use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut ans: i64 = 0;
    let mut b: Vec<i64> = vec![0; n];
    for i in 0..n {
        if a[i] == (i + 1) as i64 {
            b[i] = 1;
        }
    }
    let mut c: Vec<i64> = vec![0; n];
    c[0] = b[0];
    for i in 1..n {
        c[i] = c[i - 1] + b[i];
    }
    for i in 0..n {
        if a[i] == (i + 1) as i64 {
            ans += c[n - 1] - c[i];
        } else if i < (a[i] - 1) as usize && a[(a[i] - 1) as usize] == (i + 1) as i64 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
