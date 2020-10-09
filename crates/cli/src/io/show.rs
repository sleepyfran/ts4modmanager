use console::style;

use crate::emoji::Emoji;

/// Shows a message in green text.
pub fn show_success<S: Into<String>>(emoji: Emoji, text: S) {
    println!("{}", text_for_success(emoji, text));
}

/// Formats the text in the success format.
pub fn text_for_success<S: Into<String>>(emoji: Emoji, text: S) -> String {
    format!("{} {}", emoji, style(text.into()).green())
}

/// Shows a message in red text.
pub fn show_error<S: Into<String>>(emoji: Emoji, text: S) {
    println!("{}", text_for_error(emoji, text));
}

/// Formats the text in the error format.
pub fn text_for_error<S: Into<String>>(emoji: Emoji, text: S) -> String {
    format!("{} {}", emoji, style(text.into()).red())
}

/// Shows a message in blue text.
pub fn show_info<S: Into<String>>(emoji: Emoji, text: S) {
    println!("{}", text_for_info(emoji, text));
}

/// Formats the text in the info format.
pub fn text_for_info<S: Into<String>>(emoji: Emoji, text: S) -> String {
    format!("{} {}", emoji, style(text.into()).cyan())
}
