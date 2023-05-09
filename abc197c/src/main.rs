use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut res = 2_000_000_000;
    for i in 0..(1 << (n - 1)) {
        let mut xored = 0;
        let mut ored = 0;
        for (j, item) in a.iter().enumerate() {
            ored |= item;
            if (i >> j & 1) > 0 {
                xored ^= ored;
                ored = 0;
            }
        }
        xored ^= ored;
        res = res.min(xored);
    }
    println!("{}", res);
}
