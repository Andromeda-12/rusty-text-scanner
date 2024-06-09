#[cfg(test)]
mod test {
    use crate::{color_utils, consts::colors, utils::split_word_by_substring_inclusive};
    use std::io::Write;
    use termcolor::{Buffer, Color, ColorSpec, WriteColor};

    #[test]
    fn get_highlighted_line() {
        helper_get_highlighted_line("test string with some test text", "test");
        helper_get_highlighted_line("test string with some test text", "ne_test");
        helper_get_highlighted_line("test string with some test text", "");
        helper_get_highlighted_line("русский рандомный текст", "рандомный");
    }

    #[test]
    fn get_highlighted_line_with_case_sensitive() {
        helper_get_highlighted_line_with_case_sensitive("Test string with some tEsT text", "test");
        helper_get_highlighted_line_with_case_sensitive("Тестовый текст для теста", "тест");
        helper_get_highlighted_line_with_case_sensitive("Тестовый тест для тест теста", "тест");
        helper_get_highlighted_line_with_case_sensitive("Тестовый тест для тест теста", "");
    }

    #[test]
    fn get_highlighted_line_with_substring() {
        helper_get_highlighted_line_with_substring("1Test string with some tEsT text", "test");
        helper_get_highlighted_line_with_substring("1test string with some testtt text", "test");
        helper_get_highlighted_line_with_substring("Тестовый тест текст для теста", "тест");
        helper_get_highlighted_line_with_substring("Тестовый текст для теста", "");
    }

    #[test]
    fn get_highlighted_line_with_case_sensitive_substring() {
        helper_get_highlighted_line_with_case_sensitive_substring(
            "TestTTT string with some TTtTtEsTTT text",
            "test",
        );
        helper_get_highlighted_line_with_case_sensitive_substring(
            "Нетестовая строка, ТЕСТОВАЯ строка, НЕТЕСТОВАЯнетестовая или тестоваятестовая?",
            "тест",
        );
        helper_get_highlighted_line_with_case_sensitive_substring("тесттест ТЕСТТЕСТ", "тест");
        helper_get_highlighted_line_with_case_sensitive_substring("тесттест ТЕСТТЕСТ", "ТЕСТ");
        helper_get_highlighted_line_with_case_sensitive_substring("тесттест ТЕСТТЕСТ", "");
    }

    fn helper_get_highlighted_line(test_str: &str, search_word: &str) {
        const HIGHLIGHTED_LINE_FG_COLOR: Color = colors::HIGHLIGHTED_LINE_FG_COLOR;
        const HIGHLIGHTED_WORD_BG_COLOR: Color = colors::HIGHLIGHTED_WORD_BG_COLOR;

        let mut color_spec = ColorSpec::new();
        let mut buffer = Buffer::ansi();

        let words: Vec<&str> = test_str.split(' ').collect();
        for word in words {
            color_spec.set_fg(Some(HIGHLIGHTED_LINE_FG_COLOR));

            if word == search_word {
                color_spec.set_bg(Some(HIGHLIGHTED_WORD_BG_COLOR));
            }

            buffer.set_color(&color_spec).unwrap();
            buffer.write(word.as_bytes()).unwrap();
            buffer.reset().unwrap();
            buffer.write(b" ").unwrap();
            color_spec.clear();
        }

        let expected_string = String::from_utf8(buffer.into_inner()).unwrap();
        let received_string =
            color_utils::get_highlighted_line(test_str, search_word, false, false).unwrap();

        assert!(!received_string.is_empty(), "Received string is empty");
        assert_eq!(
            expected_string, received_string,
            "Expected string: {}, Received string: {}",
            expected_string, received_string
        );
    }

    fn helper_get_highlighted_line_with_case_sensitive(test_str: &str, search_word: &str) {
        const HIGHLIGHTED_LINE_FG_COLOR: Color = colors::HIGHLIGHTED_LINE_FG_COLOR;
        const HIGHLIGHTED_WORD_BG_COLOR: Color = colors::HIGHLIGHTED_WORD_BG_COLOR;

        let mut color_spec = ColorSpec::new();
        let mut buffer = Buffer::ansi();

        let words: Vec<&str> = test_str.split(' ').collect();
        for word in words {
            color_spec.set_fg(Some(HIGHLIGHTED_LINE_FG_COLOR));

            if word.to_lowercase() == search_word.to_lowercase() {
                color_spec.set_bg(Some(HIGHLIGHTED_WORD_BG_COLOR));
            }

            buffer.set_color(&color_spec).unwrap();
            buffer.write(word.as_bytes()).unwrap();
            buffer.reset().unwrap();
            buffer.write(b" ").unwrap();
            color_spec.clear();
        }

        let expected_string = String::from_utf8(buffer.into_inner()).unwrap();
        let received_string =
            color_utils::get_highlighted_line(test_str, search_word, true, false).unwrap();

        assert!(!received_string.is_empty(), "Received string is empty");
        assert_eq!(
            expected_string, received_string,
            "Expected string: {}, Received string: {}",
            expected_string, received_string
        );
    }

    fn helper_get_highlighted_line_with_substring(test_str: &str, search_word: &str) {
        const HIGHLIGHTED_LINE_FG_COLOR: Color = colors::HIGHLIGHTED_LINE_FG_COLOR;
        const HIGHLIGHTED_WORD_BG_COLOR: Color = colors::HIGHLIGHTED_WORD_BG_COLOR;

        let mut color_spec = ColorSpec::new();
        let mut buffer = Buffer::ansi();

        let words: Vec<&str> = test_str.split(' ').collect();

        for word in words {
            if word.contains(search_word) {
                let word_parts = split_word_by_substring_inclusive(word, search_word);

                for word_part in word_parts {
                    color_spec.set_fg(Some(HIGHLIGHTED_LINE_FG_COLOR));

                    if word_part == search_word {
                        color_spec.set_bg(Some(HIGHLIGHTED_WORD_BG_COLOR));
                    }

                    buffer.set_color(&color_spec).unwrap();
                    buffer.write(word_part.as_bytes()).unwrap();
                    buffer.reset().unwrap();
                    color_spec.clear();
                }

                buffer.write(b" ").unwrap();
                continue;
            }

            color_spec.set_fg(Some(HIGHLIGHTED_LINE_FG_COLOR));
            buffer.set_color(&color_spec).unwrap();
            buffer.write(word.as_bytes()).unwrap();
            buffer.reset().unwrap();
            buffer.write(b" ").unwrap();
            color_spec.clear();
        }

        let expected_string = String::from_utf8(buffer.into_inner()).unwrap();

        let received_string =
            color_utils::get_highlighted_line(test_str, search_word, false, true).unwrap();

        assert!(!received_string.is_empty(), "Received string is empty");
        assert_eq!(
            expected_string, received_string,
            "Expected string: {}, Received string: {}",
            expected_string, received_string
        );
    }

    fn helper_get_highlighted_line_with_case_sensitive_substring(
        test_str: &str,
        search_word: &str,
    ) {
        const HIGHLIGHTED_LINE_FG_COLOR: Color = colors::HIGHLIGHTED_LINE_FG_COLOR;
        const HIGHLIGHTED_WORD_BG_COLOR: Color = colors::HIGHLIGHTED_WORD_BG_COLOR;

        let mut color_spec = ColorSpec::new();
        let mut buffer = Buffer::ansi();

        let words: Vec<&str> = test_str.split(' ').collect();
        for word in words {
            color_spec.set_fg(Some(HIGHLIGHTED_LINE_FG_COLOR));

            if word.to_lowercase().contains(&search_word.to_lowercase()) {
                let word_parts = split_word_by_substring_inclusive(word, search_word);

                for word_part in word_parts {
                    color_spec.set_fg(Some(HIGHLIGHTED_LINE_FG_COLOR));

                    if word_part.to_lowercase() == search_word.to_lowercase() {
                        color_spec.set_bg(Some(HIGHLIGHTED_WORD_BG_COLOR));
                    }

                    buffer.set_color(&color_spec).unwrap();
                    buffer.write(word_part.as_bytes()).unwrap();
                    buffer.reset().unwrap();
                    color_spec.clear();
                }

                buffer.write(b" ").unwrap();
                continue;
            }

            color_spec.set_fg(Some(HIGHLIGHTED_LINE_FG_COLOR));
            buffer.set_color(&color_spec).unwrap();
            buffer.write(word.as_bytes()).unwrap();
            buffer.reset().unwrap();
            buffer.write(b" ").unwrap();
            color_spec.clear();
        }

        let expected_string = String::from_utf8(buffer.into_inner()).unwrap();

        let received_string =
            color_utils::get_highlighted_line(test_str, search_word, true, true).unwrap();

        assert!(!received_string.is_empty(), "Received string is empty");
        assert_eq!(
            expected_string, received_string,
            "Expected string: {}, Received string: {}",
            expected_string, received_string
        );
    }
}
