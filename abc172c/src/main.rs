use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
     k: i64,
        a: [i64; n],
        b: [i64; m]
    }
    let mut aa = vec![0; n];
    let mut bb = vec![0; m];
    aa[0] = a[0];
    bb[0] = b[0];
    for i in 1..n {
        aa[i] = aa[i - 1] + a[i];
    }
    for i in 1..m {
        bb[i] = bb[i - 1] + b[i];
    }
    let mut ans = 0;
    for i in 0..n {
        if aa[i] < k {
            let v = match bb.binary_search(&(k - aa[i])) {
                Ok(x) => x + 1,
                Err(x) => x,
            };
            ans = ans.max(i + 1 + v);
        } else {
            break;
        }
    }
    println!("{}", ans);
}
