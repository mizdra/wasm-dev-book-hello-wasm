extern {
    fn date_now() -> f64;
}

#[no_mangle]
pub fn get_timestamp() -> f64 {
    unsafe {
        date_now()
    }
}

#[no_mangle]
pub fn increment(num: i32) -> i32 {
    num + 1
}

