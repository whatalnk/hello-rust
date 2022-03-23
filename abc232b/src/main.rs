use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }
    for i in 0..27 {
        let v: Vec<u8> = s.chars().map(|x| 97 + ((x as u8 - 97 + i) % 26)).collect();
        let ss: String = String::from_utf8(v).unwrap();
        if ss == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
