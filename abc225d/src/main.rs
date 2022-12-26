use proconio::input;

fn main() {
    input!(n: usize, q: usize);
    println!("{} {}", n, q);
    for _ in 0..q {
        input!(c: usize);
        match c {
            1 => {
                input!(x: usize, y: usize);
                println!("{} {}", x, y);
            }
            2 => {
                input!(x: usize, y: usize);
                println!("{} {}", x, y);
            }
            _ => {
                input!(x: usize);
                println!("{}", x);
            }
        }
    }
}
