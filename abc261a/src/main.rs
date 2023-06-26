use proconio::input;

fn main() {
    input! {
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize
    }
    let mut v = vec![0; 101];
    for vi in v.iter_mut().take(r1 + 1).skip(l1) {
        *vi += 1;
    }
    for vi in v.iter_mut().take(r2 + 1).skip(l2) {
        *vi += 1;
    }
    let mut ans = 0;
    for vi in v.iter().take(101) {
        if vi == &2 {
            ans += 1;
        }
    }
    if ans > 0 {
        println!("{}", ans - 1);
    } else {
        println!("0");
    }
}
