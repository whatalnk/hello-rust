use proconio::input;

struct S {
    n: usize,
    a: Vec<Vec<i64>>,
}

impl S {
    fn calc(&self, used: &mut Vec<bool>, v: &mut Vec<(usize, usize)>) -> i64 {
        if v.len() == self.n {
            let mut ret: i64 = 0;
            for vi in v.iter().take(self.n) {
                ret ^= self.a[vi.0][vi.1];
            }
            return ret;
        }

        let mut l: usize = 0;
        for (i, usedi) in used.iter().enumerate().take(2 * self.n) {
            if !usedi {
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
        ret
    }
}

fn main() {
    input!(n: usize);
    let mut a = vec![vec![0; 2 * n]; 2 * n];
    for i in 1..2 * n {
        input!(aa: [i64; 2 * n - i]);
        a[i - 1][i..((2 * n - i) + i)].clone_from_slice(&aa[..(2 * n - i)]);
    }
    let s = S { n, a };
    println!("{}", s.calc(&mut vec![false; 20], &mut vec![]));
}
