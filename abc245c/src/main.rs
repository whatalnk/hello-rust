use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n]
    }
    let mut x = vec![0; n];
    let mut ans1 = true;
    x[0] = a[0];
    for i in 0..(n - 1) {
        if (x[i] - a[i + 1]).abs() <= k {
            x[i + 1] = a[i + 1];
        } else if (x[i] - b[i + 1]).abs() <= k {
            x[i + 1] = b[i + 1];
        } else {
            ans1 = false;
            break;
        }
    }
    println!("{:?}", x);
    let mut x = vec![0; n];
    let mut ans2 = true;
    x[0] = b[0];
    for i in 0..(n - 1) {
        if (x[i] - b[i + 1]).abs() <= k {
            x[i + 1] = b[i + 1];
        } else if (x[i] - a[i + 1]).abs() <= k {
            x[i + 1] = a[i + 1];
        } else {
            ans2 = false;
            break;
        }
    }
    println!("{:?}", x);
    if ans1 || ans2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
