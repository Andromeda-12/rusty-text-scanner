#[cfg(test)]
mod test {
    use crate::{color_utils, consts::colors};
    use std::io::Write;
    use termcolor::{Buffer, Color, ColorSpec, WriteColor};

    #[test]
    fn test_get_colored_slice_with_yellow_bg() {
        const TEST_STR: &str = "test";
        const COLOR: Color = colors::YELLOW;
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
}
