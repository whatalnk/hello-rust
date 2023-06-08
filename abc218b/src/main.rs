use proconio::input;

fn main() {
    input! {
        p: [u8; 26]
    }
    let a = b'a';
    let s: String = p.iter().map(|i| (a + i - 1) as char).collect();
    println!("{}", s);
}
