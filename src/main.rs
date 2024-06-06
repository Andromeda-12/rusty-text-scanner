mod color_utils;
mod consts;
mod test;
mod utils;

fn main() {
    color_utils::print_matched_line(
        "ТЕСТ нетест sss".to_owned(),
        vec!["1: test".to_string(), "2: test".to_string()],
        vec!["4: test".to_string(), "5: test".to_string()],
        "тест",
        false,
        true,
    );

    // let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    // let mut color_spec = ColorSpec::new();

    // for i in 1..255 {
    //     color_spec.set_fg(Some(Color::Ansi256(i)));
    //     stdout.set_color(&color_spec);
    //     let _ = stdout.write("text\n".as_bytes());
    // }
}
