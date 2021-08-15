#[no_mangle]
pub extern "C" fn get_sum(a: u32, b: u32) -> u32 {
    a + b
}