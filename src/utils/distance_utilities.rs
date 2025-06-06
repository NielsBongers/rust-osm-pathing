pub const F64_TO_U64_MULTIPLIER: f64 = 1e9;

pub fn f64_to_u64(input: f64) -> u64 {
    (input * F64_TO_U64_MULTIPLIER) as u64
}

pub fn u64_to_f64(input: u64) -> f64 {
    input as f64 / F64_TO_U64_MULTIPLIER
}
