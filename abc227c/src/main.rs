use proconio::input;

fn main() {
    input! {
        n: i128
    }
    let mut amax = n;
    for i in 1..=n {
        if i.pow(3) >= n {
            amax = i;
            break;
        }
    }
    // println!("{}", amax);
    let mut ans = 0;
    for a in 1..=amax {
        for b in a..=(n / (a * a)) {
            let c = 0.max((n / (a * b)) - b + 1);
            // println!("{} x {}: {}", a, b, c);
            ans += c;
        }
    }
    println!("{}", ans);
}
