pub fn reply(message: &str) -> &str {
    let is_empty = message.trim().is_empty();
    let is_yelling =
        message.chars().any(char::is_uppercase) && !message.chars().any(char::is_lowercase);
    let is_question = message.trim().ends_with('?');

    match (is_yelling, is_question, is_empty) {
        (true, false, _) => "Whoa, chill out!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
