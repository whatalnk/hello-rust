use proconio::input;

fn main() {
    input! {
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize
    }
    let mut v = vec![0; 101];
    for i in l1..=r1 {
        v[i] += 1;
    }
    for i in l2..=r2 {
        v[i] += 1;
    }
    let mut ans = 0;
    for i in 0..101 {
        if v[i] == 2 {
            ans += 1;
        }
    }
    if ans > 0 {
        println!("{}", ans - 1);
    } else {
        println!("0");
    }
}
