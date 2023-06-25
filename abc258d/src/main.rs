use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(i64, i64); n]
    }
    let mut aa = ab[0].0 + ab[0].1;
    let mut bmin = ab[0].1;
    let mut ans = aa + ((x - 1) as i64) * bmin;
    for (i, abi) in ab.iter().enumerate().take(n.min(x)).skip(1) {
        aa += abi.0 + abi.1;
        bmin = bmin.min(abi.1);
        let ans_ = aa + ((x - i - 1).max(0) as i64) * bmin;
        ans = ans.min(ans_);
    }
    println!("{}", ans);
}
