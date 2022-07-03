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
    for i in 1..(n.min(x)) {
        aa += ab[i].0 + ab[i].1;
        bmin = bmin.min(ab[i].1);
        let ans_ = aa + ((x - i - 1).max(0) as i64) * bmin;
        ans = ans.min(ans_);
    }
    println!("{}", ans);
}
