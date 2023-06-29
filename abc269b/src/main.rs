use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 10]
    }
    let mut ii = vec![];
    let mut jj = vec![];
    for (i, si) in s.iter().enumerate().take(10) {
        for (j, sij) in si.iter().enumerate().take(10) {
            if sij == &'#' {
                ii.push(i + 1);
                jj.push(j + 1);
            }
        }
    }
    ii.sort();
    jj.sort();
    let a = ii.first().unwrap();
    let b = ii.last().unwrap();
    let c = jj.first().unwrap();
    let d = jj.last().unwrap();
    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
