use std::collections::VecDeque;
use text_io::read;

fn main() {
    let n: usize = read!();
    let mut vx: VecDeque<i64> = VecDeque::new();
    let mut vc: VecDeque<i64> = VecDeque::new();
    for _ in 0..n {
        let q: usize = read!();
        if q == 1 {
            let x: i64 = read!();
            let c: i64 = read!();
            vx.push_back(x);
            vc.push_back(c);
        } else {
            let mut c: i64 = read!();
            let mut s = 0;
            while c > 0 {
                let cc = vc.pop_front().unwrap();
                let xx = vx.pop_front().unwrap();
                if cc > c {
                    s += xx * c;
                    vc.push_front(cc - c);
                    vx.push_front(xx);
                    c = 0;
                } else if cc == c {
                    s += xx * c;
                    c = 0;
                } else {
                    s += xx * cc;
                    c -= cc;
                }
            }
            println!("{}", s)
        }
    }
}
