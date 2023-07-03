use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

fn valid(x: isize, y: isize, s: &[Vec<char>]) -> bool {
    let xx = x.min(8).max(0);
    let yy = y.min(8).max(0);
    xx == x && yy == y && s[xx as usize][yy as usize] == '#'
}

fn main() {
    input! {
        s: [Chars; 9],
    }
    let mut hs = BTreeSet::<BTreeSet<(isize, isize)>>::new();
    for i in 0..9 {
        for j in 0..9 {
            for dx in -8..9 {
                for dy in -8..9 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let i2 = i as isize + dx;
                    let j2 = j as isize + dy;
                    let i3 = i2 - dy;
                    let j3 = j2 + dx;
                    let i4 = i3 - dx;
                    let j4 = j3 - dy;
                    if valid(i, j, &s)
                        && valid(i2, j2, &s)
                        && valid(i3, j3, &s)
                        && valid(i4, j4, &s)
                    {
                        let mut hss = BTreeSet::<(isize, isize)>::new();
                        hss.insert((i, j));
                        hss.insert((i2, j2));
                        hss.insert((i3, j3));
                        hss.insert((i4, j4));
                        hs.insert(hss);
                    }
                }
            }
        }
    }

    println!("{}", hs.len());
}
