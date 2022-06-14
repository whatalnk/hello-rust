use proconio::input;

fn main() {
    input! {
        x: i128,
        mut a: i128,
        mut d: i128,
        n: i128
    }
    if d < 0 {
        a = a + d * (n - 1);
        d = -d;
    }
    let s0 = a;
    let sn = a + d * (n - 1);
    if x >= s0 && x <= sn {
        if d == 0 {
            println!("0");
        } else {
            let ans = (x - a) % d;
            println!("{}", ans.min(d - ans));
        }
    } else if x < s0 {
        println!("{}", s0 - x);
    } else {
        println!("{}", x - sn);
    }
}
