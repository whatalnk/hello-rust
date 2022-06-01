use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut k: i64,
        a: [i64; n],
        b: [i64; m]
    }
    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    loop {
        if i < n && j < m {
            if a[i] < b[j] && k >= a[i] {
                k -= a[i];
                ans += 1;
                i += 1;
            } else if a[i] >= b[j] && k >= b[j] {
                k -= b[j];
                ans += 1;
                j += 1;
            } else {
                break;
            }
        } else if i < n && j >= m {
            if k >= a[i] {
                k -= a[i];
                ans += 1;
                i += 1;
            } else {
                break;
            }
        } else if i >= n && j < m {
            if k >= b[j] {
                k -= b[j];
                ans += 1;
                j += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    println!("{}", ans);
}
