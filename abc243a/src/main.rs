use proconio::input;

fn main() {
    input! {
        mut v: i32,
        a: i32,
        b: i32,
        c: i32
    }
    while v >= 0 {
        if v < a {
            println!("F");
            return;
        } else if v < a + b {
            println!("M");
            return;
        } else if v < a + b + c {
            println!("T");
            return;
        } else {
            v -= a + b + c;
        }
    }
}
