pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut res = vec![];

    if len > digits.len() {
        return res;
    }

    for i in 0..=(digits.len() - len) {
        res.push(String::from(&digits[i..i+len]));
    }

    return res;
}
