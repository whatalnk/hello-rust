use proconio::input;

fn main() {
    input! {
        t: usize,
        aa: [(i64, i64); t]
    }
    for i in 0..t {
        let (a, s) = aa[i];
        let mut ans = false;
        if 2 * a <= s {
            let differ = s - 2 * a;
            if (differ & a) == 0 {
                ans = true;
            }
        }
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
