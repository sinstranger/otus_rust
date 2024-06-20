#[no_mangle]
pub extern "C" fn hello(x: i32) -> i32 {
    x * x
}
