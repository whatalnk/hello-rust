use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut even = vec![];
    let mut odd = vec![];
    for ai in a.iter().take(n) {
        if ai % 2 == 0 {
            even.push(ai);
        } else {
            odd.push(ai);
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
