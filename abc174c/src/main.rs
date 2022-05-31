use proconio::input;

fn main() {
    input! {
        k: i64
    }
    if k % 2 == 0 || k % 5 == 0 {
        println!("-1");
        return;
    } else {
        let mut l = 9 * k;
        if k % 7 == 0 {
            l /= 7;
        }
        let mut x = 10;
        for i in 1..l {
            if x % l == 1 {
                println!("{}", i);
                return;
            }
            x *= 10;
            x %= l;
        }
    }
    println!("-1");
}
