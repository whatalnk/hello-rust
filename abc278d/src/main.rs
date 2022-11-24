use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!(n: usize);
    input!(a: [i64; n]);
    input!(q: usize);
    println!("{}", n);
    println!("{:?}", a);
    println!("{}", q);
    for _ in 0..q {
        input!(c: usize);
        match c {
            1 => {
                input!(x: i64);
                println!("{} {}", c, x);
            }
            2 => {
                input!(i: usize);
                input!(x: i64);
                println!("{} {} {}", c, i, x);
            }
            _ => {
                input!(i: usize);
                println!("{} {}", c, i);
            }
        }
    }
}
