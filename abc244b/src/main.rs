use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut x = 0;
    let mut y = 0;
    let mut d = 'E';
    let mut dx = 1;
    let mut dy = 0;
    for i in 0..n {
        if s[i] == 'S' {
            x += dx;
            y += dy;
        } else {
            if d == 'E' {
                d = 'S';
                dx = 0;
                dy = -1;
            } else if d == 'S' {
                d = 'W';
                dx = -1;
                dy = 0;
            } else if d == 'W' {
                d = 'N';
                dx = 0;
                dy = 1;
            } else {
                d = 'E';
                dx = 1;
                dy = 0;
            }
        }
    }
    println!("{} {}", x, y);
}
