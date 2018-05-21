pub fn twofer(name: &str)-> String {
    let mut s = name;

    if "" == s {
        s = "you";
    }

    format!("One for {}, one for me.", s)
}
