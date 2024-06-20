fn func(a: u32) -> bool {
    if a > 120 {
        return true;
    }
    false
}

fn main() {
    let x = func;

    let x = x(2);

    println!("{:?}", Result::Ok)
}
