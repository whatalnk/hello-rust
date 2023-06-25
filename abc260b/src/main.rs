use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [i64; n],
        b: [i64; n]
    }
    let mut ab = Vec::<(i64, i64, i64, usize)>::new();
    let mut ans = vec![];
    for i in 0..n {
        ab.push((a[i], b[i], a[i] + b[i], i + 1));
    }
    ab.sort_by(|a, b| b.0.cmp(&a.0));
    for _ in 0..x {
        let (_, _, _, i) = ab.remove(0);
        ans.push(i);
    }

    ab.sort_by(|a, b| a.3.cmp(&b.3));
    ab.sort_by(|a, b| b.1.cmp(&a.1));
    for _ in 0..y {
        let (_, _, _, i) = ab.remove(0);
        ans.push(i);
    }
    ab.sort_by(|a, b| a.3.cmp(&b.3));
    ab.sort_by(|a, b| b.2.cmp(&a.2));
    for _ in 0..z {
        let (_, _, _, i) = ab.remove(0);
        ans.push(i);
    }
    ans.sort();
    for ansi in &ans {
        println!("{}", ansi);
    }
}
