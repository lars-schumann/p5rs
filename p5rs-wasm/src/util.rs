pub fn log<T: std::fmt::Debug>(thing: &T) {
    web_sys::console::log_1(&format!("logging: {:?}", thing).into());
}

pub fn random_int(lower_bound: i64, upper_bound: i64) -> i64 {
    let range = upper_bound - lower_bound + 1;
    (web_sys::js_sys::Math::random() * range as f64).floor() as i64 + lower_bound
}
