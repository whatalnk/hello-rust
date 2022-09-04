use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    }
    let mut hm = HashMap::new();
    hm.insert("Monday".to_string(), 5);
    hm.insert("Tuesday".to_string(), 4);
    hm.insert("Wednesday".to_string(), 3);
    hm.insert("Thursday".to_string(), 2);
    hm.insert("Friday".to_string(), 1);
    println!("{}", hm.get(&s).unwrap());
}
