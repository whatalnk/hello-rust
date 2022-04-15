use proconio::input;

fn main() {
    let max_s = 1200;
    input! {
        n: usize,
        k: i64
    }
    let mut a = vec![];
    let mut p = vec![0; max_s + 2];
    for _ in 0..n {
        input! {
            a1: i64,
            a2: i64,
            a3: i64
        }
        let s = a1 + a2 + a3;
        a.push(s);
        p[s as usize] += 1;
    }
    let mut pp = vec![0; max_s + 2];
    pp[max_s] = p[max_s];
    for i in 0..max_s {
        pp[max_s - i - 1] = pp[max_s - i] + p[max_s - i - 1];
    }
    for s in a {
        let score = s + 300;
        let mut rank = pp[score as usize];
        if p[score as usize] == 0 {
            rank += 1;
        }
        if rank <= k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
