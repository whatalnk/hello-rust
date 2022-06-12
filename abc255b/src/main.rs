use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(f64, f64); n]
    }
    let b: Vec<usize> = (1..=n).filter(|x| !a.contains(x)).collect();
    let mut bb: Vec<f64> = vec![10000000000.0; n - k];
    for i in 0..k {
        for j in 0..(n - k) {
            let l = ((xy[a[i] - 1].0 - xy[b[j] - 1].0).powf(2.0)
                + (xy[a[i] - 1].1 - xy[b[j] - 1].1).powf(2.0))
            .sqrt();
            bb[j] = bb[j].min(l);
        }
    }
    let mut ans: f64 = 0.0;
    for j in 0..(n - k) {
        ans = ans.max(bb[j]);
    }
    println!("{}", ans);
}
