pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let contains_alphabet = message.chars().any(|c| c.is_alphabetic());

    if message.ends_with("?") && message.to_uppercase() == message && contains_alphabet{
        "Calm down, I know what I'm doing!"
    }
    else if message.ends_with("?") {
        "Sure."
    }
    else if message.to_uppercase() == message && contains_alphabet{
        "Whoa, chill out!"
    }
    else if message.is_empty() {
        "Fine. Be that way!"
    }
    else {
        "Whatever."
    }
}
