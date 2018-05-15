pub fn square(s: u32) -> u64 {
    if s <= 0 || s >= 65 {
        panic!("Square must be between 1 and 64")
    }
    2_u64.pow(s - 1)
}

// Sum of 2^0, 2^1, 2^2, ... 2^k = 2^(k + 1) - 1
pub fn total() -> u64 {
    u64::max_value()
}
