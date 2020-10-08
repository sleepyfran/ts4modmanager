use console::style;

use crate::emoji::Emoji;

/// Shows a message in green text.
pub fn show_success<S: Into<String>>(emoji: Emoji, text: S) {
    println!("{} {}", emoji, style(text.into()).green());
}

/// Shows a message in red text.
pub fn show_error<S: Into<String>>(emoji: Emoji, text: S) {
    println!("{} {}", emoji, style(text.into()).red());
}

/// Shows a message in blue text.
pub fn show_info<S: Into<String>>(emoji: Emoji, text: S) {
    println!("{} {}", emoji, style(text.into()).cyan());
}
