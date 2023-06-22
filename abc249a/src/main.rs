use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: usize,
        b: i32,
        c: usize,
        d: usize,
        e: i32,
        f: usize,
        x: usize
    }
    let mut tak = vec![0; x + 1];
    let mut aok = vec![0; x + 1];
    let mut cnt = 0;
    'outer1: loop {
        for _ in 0..a {
            if cnt > x {
                break 'outer1;
            }
            tak[cnt] = b;
            cnt += 1;
        }
        for _ in 0..c {
            if cnt > x {
                break 'outer1;
            }
            tak[cnt] = 0;
            cnt += 1;
        }
    }
    let mut cnt = 0;
    'outer2: loop {
        for _ in 0..d {
            if cnt > x {
                break 'outer2;
            }
            aok[cnt] = e;
            cnt += 1;
        }
        for _ in 0..f {
            if cnt > x {
                break 'outer2;
            }
            aok[cnt] = 0;
            cnt += 1;
        }
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..x {
        ans1 += tak[i];
        ans2 += aok[i];
    }
    match ans1.cmp(&ans2) {
        Ordering::Greater => {
            println!("Takahashi");
        }
        Ordering::Equal => {
            println!("Draw");
        }
        Ordering::Less => {
            println!("Aoki");
        }
    }
}
