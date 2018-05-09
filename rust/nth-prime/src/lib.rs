pub fn is_prime(p: u32) -> bool {
    let limit = (p as f64).sqrt();
    let limit = limit as u32 + 1; 

    for x in 2..limit  {
        if x % 2 == 0 && x != 2 {
            continue;
        }

        if p % x == 0 {
            return false;
        }
    }

    return true;
}

pub fn nth(n: u32) -> Option<u32> {
    if n <= 0 {
        return None;
    }

    let mut p = 2;
    let mut i = 1;

    while i < n {
        if p == 2 {
            p += 1;
        } else {
            p += 2;
        }

        while !is_prime(p) {
            if p == 2 {
                p += 1;
            } else {
                p += 2;
            }
        }

        i += 1;
    }

    return Some(p);
}
