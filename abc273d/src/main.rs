use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: usize,
        cs: usize,
        n: usize,
        rc: [(usize, usize); n],
        q: usize,
        dl: [(char, usize); q],
    }
    let mut m = vec![vec![0; w + 1]; h + 1];
    println!("{} {}", h, w);
    println!("{} {}", rs, cs);
    println!("{}", n);
    for i in 0..n {
        let (r, c) = rc[i];
        m[r][c] = 1;
    }
    for i in 0..=h {
        println!("{:?}", m[i]);
    }
    println!("{}", q);
    println!("{:?}", dl);
}
