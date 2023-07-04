use proconio::input;

struct S {
    color: Vec<i64>,
    g: Vec<Vec<usize>>,
}
impl S {
    fn new(g: Vec<Vec<usize>>) -> S {
        S {
            color: vec![0; 200_005],
            g,
        }
    }
    fn dfs(&mut self, v: usize, p: i64, c: i64) -> (i64, i64) {
        let mut ret = (0, 0);
        self.color[v] = c;
        if c == 1 {
            ret.0 += 1;
        } else {
            ret.1 += 1;
        }
        for i in 0..(self.g[v].len()) {
            let u = self.g[v][i];
            if u == p as usize || self.color[u] == -c {
                continue;
            }
            if self.color[u] == c {
                return (-1, -1);
            }
            let res = self.dfs(u, v as i64, -c);
            if res.0 == -1 {
                return (-1, -1);
            }
            ret.0 += res.0;
            ret.1 += res.1
        }
        ret
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut g = vec![vec![]; 200_005];
    for uvi in uv.iter().take(m) {
        let (u, v) = uvi;
        g[*u].push(*v);
        g[*v].push(*u);
    }
    let mut ans: i64 = (n * (n - 1) / 2 - m) as i64;
    let mut s = S::new(g);
    for i in 1..=n {
        if s.color[i] == 0 {
            let res = s.dfs(i, -1, 1);
            if res.0 == -1 {
                println!("0");
                return;
            }
            ans -= res.0 * (res.0 - 1) / 2;
            ans -= res.1 * (res.1 - 1) / 2;
        }
    }
    println!("{}", ans);
}
