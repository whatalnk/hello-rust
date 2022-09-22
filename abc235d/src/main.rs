use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        a: usize,
        n: usize
    }
    let mut m = 1;
    while m <= n {
        m *= 10;
    }

    let mut d = vec![-1; m];
    let mut q = VecDeque::new();
    d[1] = 0;
    q.push_back(1);

    while q.len() > 0 {
        let c = q.pop_front().unwrap();
        let dc = d[c];

        let op1 = a * c;
        if op1 < m && d[op1] == -1 {
            d[op1] = dc + 1;
            q.push_back(op1);
        }

        if c >= 10 && c % 10 != 0 {
            let mut x = 1;
            while x <= c {
                x *= 10;
            }
            let op2 = ((c % 10) * x + c) / 10;
            if op2 < m && d[op2] == -1 {
                d[op2] = dc + 1;
                q.push_back(op2);
            }
        }
    }
    println!("{}", d[n]);
}
