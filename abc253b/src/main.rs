use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut o = vec![];
    for (i, si) in s.iter().enumerate().take(h) {
        for (j, sij) in si.iter().enumerate().take(w) {
            if sij == &'o' {
                o.push((i as i32, j as i32));
            }
        }
    }
    let ans = (o[0].0 - o[1].0).abs() + (o[0].1 - o[1].1).abs();
    println!("{}", ans);
}
