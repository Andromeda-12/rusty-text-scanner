pub fn split_word_by_substring_inclusive(word: &str, substring: &str) -> Vec<String> {
    if substring.is_empty() {
        return vec![word.to_string()];
    }

    let mut result: Vec<String> = Vec::new();
    let mut start = 0;

    while let Some(pos) = word[start..].to_lowercase().find(&substring.to_lowercase()) {
        let end = start + pos;

        if !word[start..end].is_empty() {
            result.push(word[start..end].to_string());
        }

        result.push(word[end..end + substring.len()].to_string());
        start = end + substring.len();
    }

    if start < word.len() {
        result.push(word[start..].to_string());
    }

    result
}
