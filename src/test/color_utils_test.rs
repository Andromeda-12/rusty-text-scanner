#[cfg(test)]
mod test {
    use crate::{color_utils, consts::colors};
    use std::io::Write;
    use termcolor::{Buffer, Color, ColorSpec, WriteColor};

    #[test]
    fn test_get_colored_slice_with_yellow_bg() {
        const TEST_STR: &str = "test";
        const COLOR: Color = colors::HIGHLIGHTED_WORD_BG_COLOR;

        let mut color_spec = ColorSpec::new();
        let mut buffer = Buffer::ansi();
        color_spec.set_bg(Some(COLOR));
        buffer.set_color(&color_spec).unwrap();
        buffer.write(TEST_STR.as_bytes()).unwrap();
        buffer.reset().unwrap();

        let expected_string = String::from_utf8(buffer.into_inner()).unwrap();
        let received_string = color_utils::get_colored_slice(TEST_STR, COLOR).unwrap();

        assert!(!received_string.is_empty(), "Received string is empty");
        assert_eq!(
            expected_string, received_string,
            "Expected string: {:?}, Received string: {:?}",
            expected_string, received_string
        );
    }

    #[test]
    fn test_get_highlighted_line() {
        const TEST_STR: &str = "test string with some test text";
        const SEARCH_WORD: &str = "test";
        const LINE_FG_COLOR: Color = colors::HIGHLIGHTED_LINE_FG_COLOR;
        const HIGHLIGHTED_WORD_BG_COLOR: Color = colors::HIGHLIGHTED_WORD_BG_COLOR;

        let mut color_spec = ColorSpec::new();
        let mut buffer = Buffer::ansi();

        let words: Vec<&str> = TEST_STR.split(' ').collect();
        for word in words {
            color_spec.set_fg(Some(LINE_FG_COLOR));

            if word == SEARCH_WORD {
                color_spec.set_bg(Some(HIGHLIGHTED_WORD_BG_COLOR));
            }

            buffer.set_color(&color_spec).unwrap();
            buffer.write(word.as_bytes()).unwrap();
            buffer.reset().unwrap();
            buffer.write(b" ").unwrap();
            color_spec.clear();
        }

        let expected_string = String::from_utf8(buffer.into_inner()).unwrap();
        let received_string = color_utils::get_highlighted_line(TEST_STR, SEARCH_WORD).unwrap();

        assert!(!received_string.is_empty(), "Received string is empty");
        assert_eq!(
            expected_string, received_string,
            "Expected string: {:?}, Received string: {:?}",
            expected_string, received_string
        );
    }
}
