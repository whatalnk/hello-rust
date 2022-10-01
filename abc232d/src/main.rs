use proconio::input;
use proconio::marker::Chars;
use std::cell::Cell;

struct S {
    h: usize,
    w: usize,
    c: Vec<Vec<char>>,
    ans: Cell<i64>,
}

impl S {
    fn dfs(&self, i: usize, j: usize, cnt: i64) {
        let d = vec![(0, 1), (1, 0)];
        for di in 0..2 {
            let ni = i + d[di].0;
            let nj = j + d[di].1;
            if ni < self.h && nj < self.w && self.c[ni][nj] != '#' {
                self.dfs(ni, nj, cnt + 1);
            } else {
                self.ans.set(cnt.max(self.ans.get()));
            }
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let s = S {
        h: h,
        w: w,
        c: c,
        ans: Cell::new(0),
    };
    s.dfs(0, 0, 1);
    println!("{}", s.ans.get());
}
