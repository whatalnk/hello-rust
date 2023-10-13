use std::collections::HashMap;
use std::println;

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut hm = HashMap::new();
    hm.insert("tourist", 3858);
    hm.insert("ksun48", 3679);
    hm.insert("Benq", 3658);
    hm.insert("Um_nik", 3648);
    hm.insert("apiad", 3638);
    hm.insert("Stonefeang", 3630);
    hm.insert("ecnerwala", 3613);
    hm.insert("mnbvmar", 3555);
    hm.insert("newbiedmy", 3516);
    hm.insert("semiexp", 3481);
    println!("{}", hm.get(&s.as_str()).unwrap());
}
