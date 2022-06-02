use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
     k: i64,
        a: [i64; n],
        b: [i64; m]
    }
    let mut aa = vec![0; n + 1];
    let mut bb = vec![0; m + 1];
    for i in 0..n {
        aa[i + 1] = aa[i] + a[i];
    }
    for i in 0..m {
        bb[i + 1] = bb[i] + b[i];
    }
    let mut ans = 0;
    for i in 0..(n + 1) {
        if aa[i] <= k {
            let v = match bb.binary_search(&(k - aa[i])) {
                Ok(x) => x,
                Err(x) => x - 1,
            };
            ans = ans.max(i + v);
        } else {
            break;
        }
    }
    println!("{}", ans);
}
