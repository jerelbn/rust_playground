pub mod utils {
    pub fn round2dec(val: f64, dec: i32) -> f64 {
        let ten = 10.0_f64;
        let multiplier = ten.powi(dec);
        (val * multiplier).round() / multiplier
    }
}
