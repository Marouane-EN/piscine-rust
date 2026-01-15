pub fn talking(text: &str) -> &str {
    let is_empty = text.trim().is_empty();
    let is_yelling = text.chars().any(|c| c.is_alphabetic()) && text == text.to_uppercase();
    let is_question = text.trim().ends_with('?');

    match (is_empty, is_yelling, is_question) {
        (true, _, _) => "Just say something!",
        (_, true, true) => "Quiet, I am thinking!",
        (_, true, _) => "There is no need to yell, calm down!",
        (_, _, true) => "Sure.",
        _ => "Interesting",
    }
}
