use proconio::input;

fn main() {
    let m = 998_244_353;
    input! {
        n: i128
    }
    let mut ans = 0;
    for i in 0..=18 {
        let a = (10 as i128).pow(i);
        let b = (10 as i128).pow(i + 1) - 1;
        if n >= a && n <= b {
            let aa = 1;
            let bb = n - a + 1;
            let s = (aa + bb) * (bb - aa + 1) / 2;
            ans += s;
            ans %= m;
            break;
        } else {
            let aa = 1;
            let bb = b - a + 1;
            let s = (aa + bb) * (bb - aa + 1) / 2;
            ans += s;
            ans %= m;
        }
    }
    println!("{}", ans);
}
