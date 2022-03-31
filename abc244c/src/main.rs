use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize
    }
    let mut checked = vec![false; 2 * n + 2];
    let mut deq: VecDeque<usize> = VecDeque::new();
    for i in 1..=(2 * n + 1) {
        deq.push_back(i);
    }
    let a = deq.pop_front().unwrap();
    checked[a] = true;
    println!("{}", a);
    loop {
        input! {
            k: usize
        }
        if k == 0 {
            return;
        }
        checked[k] = true;
        loop {
            let a = deq.pop_front().unwrap();
            if !checked[a] {
                println!("{}", a);
                checked[a] = true;
                break;
            }
        }
    }
}
