#[no_mangle]
pub extern "C" fn soroban_log(message: *const u8) {
    let message = unsafe { std::str::from_utf8(std::slice::from_raw_parts(message, 64)).unwrap() };
    println!("{}", message);
}
