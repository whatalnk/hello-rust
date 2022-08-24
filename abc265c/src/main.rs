use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    }
    let mut ci = 0;
    let mut cj = 0;
    let mut visited = vec![vec![false; w]; h];
    loop {
        if visited[ci][cj] {
            println!("-1");
            return;
        } else {
            visited[ci][cj] = true;
            if g[ci][cj] == 'U' && ci != 0 {
                ci -= 1;
            } else if g[ci][cj] == 'D' && ci != h - 1 {
                ci += 1;
            } else if g[ci][cj] == 'L' && cj != 0 {
                cj -= 1;
            } else if g[ci][cj] == 'R' && cj != w - 1 {
                cj += 1;
            } else {
                println!("{} {}", ci + 1, cj + 1);
                return;
            }
        }
    }
}
