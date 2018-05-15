pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];

    let mut p = 2;
    let mut x = n;

    while x >= p {
        while x % p == 0 {
            x = x / p;
            res.push(p);
        }

        if p == 2 {
            p += 1;
        } else {
            p += 2;
        }
    }

    return res;
}
