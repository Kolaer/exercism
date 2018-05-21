pub fn collatz(n: u64) -> Option<u64> {
    if n <= 0 {
        return None;
    }

    let mut num = 0;

    let mut x = n;

    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        num += 1;
    }

    Some(num)
}
