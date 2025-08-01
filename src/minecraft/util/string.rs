use std::str::Chars;

/// Escape and quote strings.
///
/// This trait is meant to increase correctness when serializing strings as RCON arguments.
///
/// IMPORTANT: This trait should not be considered to provide certain safety or security guarantees.
pub trait EscapeString {
    /// Escapes a string.
    ///
    /// Escapes double quotes and backslashes.
    fn escape(&self) -> String;

    /// Returns a quoted string.
    ///
    /// - Escapes the string.
    /// - If the string contains whitespace, puts the string in outer double quotes.
    fn quote(&self) -> String;
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

    fn quote(&self) -> String {
        let s = self.as_ref();

        if s.chars().any(char::is_whitespace) {
            format!(r#""{}""#, s.escape())
        } else {
            s.escape()
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
