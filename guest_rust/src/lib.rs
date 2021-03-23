#[no_mangle]
pub extern "C" fn activate() -> u32 {
    println!("Guest Rust was activated!");

    42
}
