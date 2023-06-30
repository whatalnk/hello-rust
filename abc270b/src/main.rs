use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }
    if x > 0 && y > 0 {
        if x < y {
            println!("{}", x);
        } else if z > y {
            println!("-1");
        } else if z < 0 {
            println!("{}", x + z.abs() * 2);
        } else {
            println!("{}", x);
        }
    } else if x > 0 && y < 0 {
        println!("{}", x);
    } else if (x < 0 && y > 0) || y < x {
        println!("{}", x.abs());
    } else if z < y {
        println!("-1");
    } else if z > 0 {
        println!("{}", x.abs() + z * 2);
    } else {
        println!("{}", x.abs());
    }
}
