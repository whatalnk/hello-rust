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
    for xi in x.iter().take(q) {
        let j = match a.binary_search(xi) {
            Ok(v) => v,
            Err(v) => {
                if v > 0 {
                    v - 1
                } else {
                    v
                }
            }
        };
        let mut left = b[j];
        let mut right = b[n - 1] - b[j];
        let cnt = if xi < &a[0] {
            right += left;
            left = 0;
            0
        } else {
            (j + 1) as i64
        };
        let ans = (xi * cnt - left) + (right - xi * (n as i64 - cnt));
        println!("{}", ans)
    }
}
