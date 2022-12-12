use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i64,
        a: [i64; n],
    };
    let mut l = vec![];
    l.push(0);
    let mut total = 0;
    for i in 0..n {
        total += a[i];
        l.push(total);
    }
    l.push(total * 2);
    let k = t - t / total * total;
    let v = l.binary_search(&k);
    // println!("{:?} {:?} {}", l, v, k);
    match v {
        Ok(vv) => println!("{} {}", vv, k - l[vv - 1]),
        Err(vv) => println!("{} {}", vv, k - l[vv - 1]),
    };
}
