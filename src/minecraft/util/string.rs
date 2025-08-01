use std::borrow::Cow;
use std::str::Chars;

/// Escape and quote strings.
pub trait EscapeString {
    /// Escapes a string.
    ///
    /// Escapes double quotes and backslashes.
    fn escape(&self) -> String;

    /// Returns a quoted string.
    ///
    /// - If the string contains double quotes, escapes the string.
    /// - If the string contains whitespace or double quotes, puts the string in outer double quotes.
    fn quote(&self) -> Cow<'_, str>;
}

impl<T> EscapeString for T
where
    T: AsRef<str>,
{
    fn escape(&self) -> String {
        self.as_ref()
            .chars()
            .flat_map(|character| match character {
                '\"' => CharsOrChar::Chars("\\\"".chars()), // Escape double quotes
                '\\' => CharsOrChar::Chars("\\\\".chars()), // Escape backslashes
                _ => CharsOrChar::Char(character),          // Other characters remain unchanged
            })
            .collect()
    }

    fn quote(&self) -> Cow<'_, str> {
        let s = self.as_ref();
        let contains_whitespace = s.chars().any(char::is_whitespace);
        let contains_double_quotes = s.chars().any(|character| character == '"');

        match (contains_whitespace, contains_double_quotes) {
            (_, true) => Cow::Owned(format!(r#""{}""#, s.escape())),
            (true, false) => Cow::Owned(format!(r#""{s}""#)),
            (false, false) => Cow::Borrowed(s),
        }
    }
}

/// Iterator helper for string escaping.
enum CharsOrChar<'a> {
    Chars(Chars<'a>),
    Char(char),
}

impl Iterator for CharsOrChar<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CharsOrChar::Chars(chars) => chars.next(),
            CharsOrChar::Char(char) => Some(*char),
        }
    }
}
