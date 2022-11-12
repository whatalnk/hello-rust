use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i128,
        a: [i128; n],
    }
    let mut ok = 0i128;
    let mut ng = 1000000000000000000 / k;
    while ng - ok > 1 {
        let md = (ok + ng) / 2;
        let mut sum = 0;
        for i in 0..n {
            sum += a[i].min(md);
        }
        if sum >= k * md {
            ok = md;
        } else {
            ng = md;
        }
    }
    println!("{}", ok);
}
