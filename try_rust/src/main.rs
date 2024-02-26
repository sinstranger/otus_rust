fn main() {
    let t: &str = "qwerty";
    let r = &t;
    let d = &r;
    let x = &d;

    println!("{}", std::mem::size_of_val(t));
    println!("{}", std::mem::size_of_val(r));
    println!("{}", std::mem::size_of_val(d));
    println!("{}", std::mem::size_of_val(x));


    println!("{}", ***x)
}
