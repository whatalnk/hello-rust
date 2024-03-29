use proconio::input;

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
            x
        } else {
            self.par[x] = self.find(self.par[x]);
            self.par[x]
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
        self.find(x) == self.find(y)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut d = vec![0; n];
    let mut uf = UnionFind::new(n);
    for abi in ab.iter().take(m) {
        let (mut a, mut b) = abi;
        a -= 1;
        b -= 1;
        if uf.same(a, b) {
            println!("No");
            return;
        }
        uf.unite(a, b);
        d[a] += 1;
        d[b] += 1;
    }
    for di in d.iter().take(n) {
        if di > &2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
