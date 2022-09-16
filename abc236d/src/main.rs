use proconio::input;

struct S {
    n: usize,
    a: Vec<Vec<i64>>,
}

impl S {
    fn calc(&self, used: &mut Vec<bool>, v: &mut Vec<(usize, usize)>) -> i64 {
        if v.len() == self.n {
            let mut ret: i64 = 0;
            for i in 0..self.n {
                ret ^= self.a[v[i].0][v[i].1];
            }
            return ret;
        }

        let mut l: usize = 0;
        for i in 0..(2 * self.n) {
            if !used[i] {
                l = i;
                break;
            }
        }
        used[l] = true;

        let mut ret = 0;
        for i in 0..2 * self.n {
            if !used[i] {
                v.push((l, i));
                used[i] = true;
                ret = ret.max(self.calc(used, v));
                v.pop();
                used[i] = false;
            }
        }

        used[l] = false;
        return ret;
    }
}

fn main() {
    input!(n: usize);
    let mut a = vec![vec![0; 2 * n]; 2 * n];
    for i in 1..=(2 * n - 1) {
        input!(aa: [i64; 2 * n - i]);
        for j in 0..(2 * n - i) {
            a[i - 1][i + j] = aa[j];
        }
    }
    let s = S { n: n, a: a.clone() };
    println!("{}", s.calc(&mut vec![false; 20], &mut vec![]));
}
