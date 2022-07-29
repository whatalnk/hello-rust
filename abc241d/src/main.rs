use proconio::input;

fn main() {
    input!(n: usize);
    println!("{}", n);
    for _ in 0..n {
        input!(c: u32);
        match c {
            1 => {
                input!(x: i64);
                println!("{}", x);
            }
            2 => {
                input!(x: i64, k: usize);
                println!("{} {}", x, k);
            }
            _ => {
                input!(x: i64, k: usize);
                println!("{} {}", x, k);
            }
        }
    }
}
