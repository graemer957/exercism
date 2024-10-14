pub fn reply(message: &str) -> &str {
    let message = message.trim();

    let contains_letters = message.contains(char::is_alphabetic);
    let is_question = message.ends_with('?');
    let is_shouting = contains_letters && message == message.to_uppercase();

    match message {
        _ if message.is_empty() => "Fine. Be that way!",
        _ if is_question && is_shouting => "Calm down, I know what I'm doing!",
        _ if is_question => "Sure.",
        _ if is_shouting => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
