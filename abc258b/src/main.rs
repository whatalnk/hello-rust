use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    }
    let d = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    let mut ans = vec![];
    for i in 0..(n as isize) {
        for j in 0..(n as isize) {
            for k in 0..8 {
                let mut cnt = 0;
                let mut s = vec![];
                let mut cx = i;
                let mut cy = j;
                while cnt < n {
                    let mut nx = cx + d[k].0;
                    if nx < 0 {
                        nx = (n as isize) + nx;
                    } else if nx >= (n as isize) {
                        nx -= n as isize;
                    }
                    let mut ny = cy + d[k].1;
                    if ny < 0 {
                        ny = (n as isize) + ny;
                    } else if ny >= (n as isize) {
                        ny -= n as isize;
                    }
                    s.push(a[nx as usize][ny as usize]);
                    cx = nx as isize;
                    cy = ny as isize;
                    cnt += 1;
                }
                ans.push(s.into_iter().collect::<String>());
            }
        }
    }
    ans.sort();
    ans.reverse();
    println!("{}", ans[0]);
}
