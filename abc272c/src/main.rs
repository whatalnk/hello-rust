use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut even = vec![];
    let mut odd = vec![];
    for i in 0..n {
        if a[i] % 2 == 0 {
            even.push(a[i]);
        } else {
            odd.push(a[i]);
        }
    }
    let mut ans = 0i64;
    if even.len() >= 2 {
        even.sort();
        ans = ans.max(even[even.len() - 1] + even[even.len() - 2]);
    }
    if odd.len() >= 2 {
        odd.sort();
        ans = ans.max(odd[odd.len() - 1] + odd[odd.len() - 2]);
    }
    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
