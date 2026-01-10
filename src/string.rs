const SEPARATORS: &[char] = &['_', '-', ' '];
const NEWLINE_CHARS: &[char] = &['\n', '\r'];

#[inline]
pub fn split_text(text: &str, sep: &str) -> Vec<String> {
    if sep.is_empty() {
        return vec![text.to_string()];
    }

    text.split(sep).map(|s| s.to_string()).collect()
}

#[inline]
pub fn join_text(text: &[&str], sep: &str) -> String {
    text.join(sep)
}

#[inline]
pub fn trim_end_matches<'a>(text: &'a str, matched_chars: &[char]) -> &'a str {
    text.trim_end_matches(matched_chars)
}

#[inline]
pub fn trim_newline(text: &str) -> &str {
    trim_end_matches(text, NEWLINE_CHARS)
}

pub fn to_pascal_case(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut pascal = String::with_capacity(text.len());
    let mut capitalize_next = true;

    for c in text.chars() {
        if SEPARATORS.contains(&c) {
            capitalize_next = true;
        } else if capitalize_next {
            pascal.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            pascal.push(c);
        }
    }

    pascal
}
