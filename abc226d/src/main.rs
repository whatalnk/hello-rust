use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    println!("{}", n);
    for i in 0..n {
        let x = xy[i].0;
        let y = xy[i].1;
        println!("{} {}", x, y);
    }
}
