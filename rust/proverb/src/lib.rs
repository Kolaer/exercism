pub fn build_proverb(list: Vec<&str>) -> String {
    let mut res = String::new();

    for i in 0..list.len() {
        if i + 1 == list.len() {
            break;
        }

        res += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1])
    }

    if list.len() > 0 {
        res += &format!("And all for the want of a {}.", list[0])
    }

    res
}
