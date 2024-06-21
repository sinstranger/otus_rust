#[no_mangle]
pub extern "C" fn rust_counter(x: i32) -> i32 {
    let mut start = 0;
    while start < x {
        start = start + 1;
    }
    start
}
