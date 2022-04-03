use proconio::input;

fn main() {
    input! {
        p1: (i32, i32),
        p2: (i32, i32),
        p3: (i32, i32),
    }
    if p1.0 == p2.0 {
        if p1.1 == p3.1 {
            println!("{} {}", p3.0, p2.1);
        } else {
            println!("{} {}", p3.0, p1.1);
        }
    } else if p1.0 == p3.0 {
        if p1.1 == p2.1 {
            println!("{} {}", p2.0, p3.1);
        } else {
            println!("{} {}", p2.0, p1.1);
        }
    } else {
        // p2.0 == p3.0
        if p2.1 == p1.1 {
            println!("{} {}", p1.0, p3.1);
        } else {
            println!("{} {}", p1.0, p2.1);
        }
    }
}
