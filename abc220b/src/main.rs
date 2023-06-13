use proconio::input;

fn base10(s: String, base: u64) -> u64 {
    let mut ret = 0;
    for x in s.chars() {
        ret *= base;
        ret += x.to_string().parse::<u64>().unwrap();
    }
    ret
}

fn main() {
    input! {
        n: u64,
        a: String,
        b: String
    }
    let aa = base10(a, n);
    let bb = base10(b, n);
    println!("{}", aa * bb);
}
