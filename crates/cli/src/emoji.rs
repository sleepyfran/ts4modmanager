use std::fmt::{Display, Formatter, Result};

/// Encapsulates a string that holds an emoji. For type-checking purposes, doesn't really check
/// that whatever walue given to the string is a valid emoji. In any case the emoji contained here
/// should at least for sure take enough space for spaces to render correctly around it.
pub struct Emoji(String);

impl Display for Emoji {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

/// Returns the assigned emoji for info.
pub fn for_sucess() -> Emoji {
    Emoji("✅".into())
}

/// Returns the assigned emoji for warnings.
pub fn for_warning() -> Emoji {
    Emoji("⚠️ ".into())
}

/// Returns the assigned emoji for questions.
pub fn for_question() -> Emoji {
    Emoji("🤨".into())
}

/// Returns the assigned emoji for errors.
pub fn for_error() -> Emoji {
    Emoji("❌".into())
}

/// Returns the assigned emoji for search.
pub fn for_search() -> Emoji {
    Emoji("🔍".into())
}

/// Returns the assigned emoji for unsuccessful (or sad) events.
pub fn for_unsuccessful() -> Emoji {
    Emoji("☹️ ".into())
}

/// Returns the assigned emoji for the parse operation.
pub fn for_fetching() -> Emoji {
    Emoji("🚚".into())
}

/// Returns the assigned emoji for the parse operation.
pub fn for_parsing() -> Emoji {
    Emoji("🔬".into())
}
