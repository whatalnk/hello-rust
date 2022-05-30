use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut res = 2000000000;
    for i in 0..(1 << (n - 1)) {
        let mut xored = 0;
        let mut ored = 0;
        for j in 0..=n {
            if j < n {
                ored |= a[j];
            }
            if j == n || (i >> j & 1) > 0 {
                xored ^= ored;
                ored = 0;
            }
        }
        res = res.min(xored);
    }
    println!("{}", res);
}
