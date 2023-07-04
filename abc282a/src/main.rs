use proconio::input;

fn main() {
    input! {
        k: u8,
    };
    let a = b'A';
    let mut ans = "".to_string();
    for i in 0..k {
        ans += &((a + i) as char).to_string();
    }
    println!("{}", ans);
}
