#[unsafe(no_mangle)]
pub unsafe extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
