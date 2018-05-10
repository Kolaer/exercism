pub fn verse(n: i32) -> String {
    if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    let n_bottles = bottles(n);
    let n1_bottles = bottles(n - 1);

    let s = if n == 1 { "it" } else { "one" };

    format!("{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n", n_bottles, n_bottles, s, n1_bottles)
}

pub fn bottles(n: i32) -> String {
    match n {
        1 => String::from("1 bottle"),
        _ if n <= 0 => String::from("no more bottles"),
        _ => format!("{} bottles", n),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..start + 1)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<String>>()
        .join("\n")
}
