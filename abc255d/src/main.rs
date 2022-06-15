use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        x: [i64; q]
    }
    a.sort();
    let mut b = vec![];
    b.push(a[0]);
    for i in 1..n {
        b.push(b[i - 1] + a[i]);
    }
    for i in 0..q {
        let mut j = match a.binary_search(&x[i]) {
            Ok(v) => v,
            Err(v) => v,
        };
        if j >= n {
            j -= 1;
        }
        let mut left = b[j];
        let mut right = b[n - 1] - b[j];
        let mut cnt = (j + 1) as i64;
        if x[i] < a[0] {
            right += left;
            left = 0;
            cnt = 0;
        }
        let ans = (x[i] * cnt - left) + (right - x[i] * (n as i64 - cnt));
        println!("{}", ans)
    }
}
