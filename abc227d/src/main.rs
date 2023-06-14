use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i128,
        a: [i128; n],
    }
    let mut ok = 0i128;
    let mut ng = 1_000_000_000_000_000_000 / k;
    while ng - ok > 1 {
        let md = (ok + ng) / 2;
        let mut sum = 0;
        for ai in a.iter().take(n) {
            sum += ai.min(&md);
        }
        if sum >= k * md {
            ok = md;
        } else {
            ng = md;
        }
    }
    println!("{}", ok);
}
