pub fn is_leap_year(year: i32) -> bool {
    match (year % 400, year % 100, year % 4) {
        (0, 0, 0) => true,
        (_, 0, 0) => false,
        (_, _, 0) => true,
        _         => false
    }
}
