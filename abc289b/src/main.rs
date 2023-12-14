use std::println;

use proconio::input;

struct S {
    n: usize,
    g: Vec<usize>,
    ans: Vec<String>,
}
impl S {
    fn new(n: usize, a: &[usize]) -> S {
        let mut g = vec![0; n];
        let ans = vec![];
        for i in a.iter() {
            g[i - 1] = 1;
        }
        S { n, g, ans }
    }
    fn dfs(&mut self, i: usize, mut c: Vec<usize>) {
        if i == self.n {
            return;
        }
        if self.g[i] == 1 {
            c.push(i + 1);
            self.dfs(i + 1, c);
        } else {
            for cc in c.iter().rev() {
                self.ans.push((cc + 1).to_string());
            }
            self.dfs(i + 1, vec![i + 1]);
        }
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut s = S::new(n, &a);
    s.dfs(0, vec![0]);
    println!("{}", s.ans.join(" "));
}
