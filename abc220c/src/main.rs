use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i128; n],
        x: i128
    }
    let sum_a: i128 = a.iter().sum();
    let mut ans = x / sum_a;
    let mut cur = ans * sum_a;
    ans *= n as i128;
    for ai in a.iter().take(n) {
        if cur + ai <= x {
            cur += ai;
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans + 1);
}
