use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1]
    }
    let mut cnt = vec![0; n];
    for (a, b) in &ab {
        cnt[a - 1] += 1;
        cnt[b - 1] += 1;
    }
    for c in cnt {
        if c == 1 || c == n - 1 {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
