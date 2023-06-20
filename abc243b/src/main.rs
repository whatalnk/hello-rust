use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n]
    }
    let mut ans1 = 0;
    for i in 0..n {
        if a[i] == b[i] {
            ans1 += 1;
        }
    }
    println!("{}", ans1);
    let mut ans2 = 0;
    for (i, ai) in a.iter().enumerate().take(n) {
        for (j, bj) in b.iter().enumerate().take(n) {
            if i != j && ai == bj {
                ans2 += 1;
            }
        }
    }
    println!("{}", ans2);
}
