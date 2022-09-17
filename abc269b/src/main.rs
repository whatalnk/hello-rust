use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 10]
    }
    let mut ii = vec![];
    let mut jj = vec![];
    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
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
