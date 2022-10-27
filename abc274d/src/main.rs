use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        a: [i64; n]
    }
    let m = 10000;
    let mut dp1 = vec![false; 2 * m + 1];
    let mut dp2 = vec![false; 2 * m + 1];
    let mut dp3 = vec![0; 2 * m + 1];
    dp1[a[0]] = true;
    dp2[0] = true;
    for i in 1..n {
        let aa = a[i];
        if i % 2 == 0 {
            for j in -m..=(m - a) {
                dp3[j + a] |= dp1[j];
                dp3[j] |= dp1[j + a];
            }
        // ???
        // swap(dp1, dp3);
        } else {
            for j in -m..=(m - a) {
                dp3[j + a] |= dp2[j];
                dp3[j] |= dp2[j + a];
            }
            // ???
            // swap[dp2, dp3];
        }
    }
}
