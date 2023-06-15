#![allow(clippy::many_single_char_names)]

use proconio::input;

struct ModUnionFind {
    par: Vec<usize>,
}
impl ModUnionFind {
    fn new(n: usize) -> ModUnionFind {
        ModUnionFind {
            par: (0..n).collect::<Vec<usize>>(),
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.find(self.par[x]);
            self.par[x]
        }
    }
    fn update_par(&mut self, x: usize, y: usize) {
        self.par[x] = y;
    }
}

fn main() {
    input! {
        q: usize,
        tx: [(i64, usize); q],
    }
    let n: usize = 1 << 20;
    let mut v: Vec<i64> = vec![-1; n];
    let mut uf = ModUnionFind::new(n);
    for txi in tx.iter().take(q) {
        let (t, x) = txi;
        if t == &1 {
            let j = uf.find(x % n);
            let k = uf.find((j + 1) % n);
            v[j] = *x as i64;
            uf.update_par(j, k);
        } else {
            println!("{}", v[x % n]);
        }
    }
}
