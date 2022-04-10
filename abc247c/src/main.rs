use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![1];
    for i in 2..=n {
        let mut b = Vec::new();
        for x in &a {
            b.push(*x);
        }
        b.push(i);
        for x in &a {
            b.push(*x);
        }
        a = b;
    }
    let a: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", a.join(" "));
}
