use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
        a: [isize; n]
    }
    let m = 10000isize;
    let mut dp1 = vec![false; 2 * (m as usize) + 1];
    let mut dp2 = vec![false; 2 * (m as usize) + 1];
    dp1[(m + a[0]) as usize] = true;
    dp2[m as usize] = true;
    for i in 1..n {
        let mut dp3 = vec![false; 2 * (m as usize) + 1];
        let aa = a[i];
        if i % 2 == 0 {
            for j in -m..=(m - aa) {
                dp3[(m + j + aa) as usize] |= dp1[(m + j) as usize];
                dp3[(m + j) as usize] |= dp1[(m + j + aa) as usize];
            }
            dp1 = dp3.to_vec();
        } else {
            for j in -m..=(m - aa) {
                dp3[(m + j + aa) as usize] |= dp2[(m + j) as usize];
                dp3[(m + j) as usize] |= dp2[(m + j + aa) as usize];
            }
            dp2 = dp3.to_vec();
        }
    }
    // println!("{:?}", dp1);
    // println!("{:?}", dp2);
    if dp1[(m + x) as usize] && dp2[(m + y) as usize] {
        println!("Yes");
    } else {
        println!("No");
    }
}
