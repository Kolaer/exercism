pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = vec![];

    let mut x = num;

    while x != 0 {
        digits.push(x % 10);
        x /= 10;
    }

    let n = digits.len() as u32;

    let sum = digits.into_iter()
                    .map(|x| x.pow(n))
                    .fold(0, |acc, x| acc + x);

    sum == num
}
