pub fn raindrops(n: usize) -> String {
    let mut s = String::new();

    if n % 3 == 0 {
        s += "Pling";
    }

    if n % 5 == 0 {
        s += "Plang";
    }

    if n % 7 == 0 {
        s += "Plong";
    }

    if s.is_empty() {
        s = n.to_string();
    }

    return s;
}
