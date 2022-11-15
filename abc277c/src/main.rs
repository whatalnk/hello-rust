use proconio::input;
use std::collections::{HashMap, HashSet};

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<i64>,
}
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect::<Vec<usize>>(),
            rank: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        } else {
            self.par[x] = self.find(self.par[x]);
            return self.par[x];
        }
    }
    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        return self.find(x) == self.find(y);
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let h = 1000000000 + 1;
    let mut uf = UnionFind::new(h);
    for i in 0..n {
        let (a, b) = ab[i];
        uf.unite(a, b);
    }
    let ans = (0..h).filter(|i| uf.par[*i] == 1).max();
    println!("{:?}", ans);
}
