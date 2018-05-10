pub fn reply(message: &str) -> &str {
    match (is_question(message), is_shout(message)) {
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Whoa, chill out!",
        (true, false) => "Sure.",
        _ if is_empty(message) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

pub fn is_empty(message: &str) -> bool {
    let trimmed = message.trim();

    trimmed.is_empty()
}

pub fn ends_with(message: &str, pat: &str) -> bool {
    let trimmed = message.trim();

    trimmed.ends_with(pat)
}

pub fn is_question(message: &str) -> bool {
    ends_with(message, "?")
}

pub fn is_shout(message: &str) -> bool {
    let trimmed = message.trim();

    let s = trimmed
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    if s.len() > 0 {
        s.chars().all(|c| c.is_uppercase())
    } else {
        false
    }
}
