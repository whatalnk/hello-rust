use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        b: [usize; k]
    }
    let amax = a.iter().max().unwrap();
    let mut ans = false;
    for i in b {
        if &a[i - 1] == amax {
            ans = true;
            break;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
