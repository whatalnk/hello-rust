use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }
    let mut q = vec![0; n];
    for i in 1..=n {
        q[p[i - 1] - 1] = i;
    }
    let ans = q
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
