use std::{
    io::{self, Write},
    ops::Range,
};

use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{
    consts::consts::colors,
    types::{LineData, WriterTrait},
};

pub struct Writer {
    stdout: StandardStream,
    color_spec: ColorSpec,
}

impl WriterTrait for Writer {
    fn new() -> Self {
        Self {
            stdout: StandardStream::stdout(ColorChoice::Always),
            color_spec: ColorSpec::new(),
        }
    }

    fn print_painted_line_with_highlighted_substring(&mut self, line: &LineData) -> io::Result<()> {
        let mut last_end = 0;

        self.color_spec
            .set_fg(Some(colors::LINE_NUMBER_FG_COLOR));
        self.stdout.set_color(&self.color_spec)?;
        self.stdout
            .write(format!("{}: ", &line.line_number).as_bytes())?;

        let result: Result<(), std::io::Error> =
            line.indices
                .iter()
                .try_for_each(|highlighted_substring_range: &Range<usize>| {
                    self.color_spec
                        .set_fg(Some(colors::HIGHLIGHTED_LINE_FG_COLOR));
                    self.stdout.set_color(&self.color_spec)?;
                    self.stdout.write(
                        &line.string.as_bytes()[last_end..highlighted_substring_range.start],
                    )?;
                    self.color_spec.clear();
                    self.stdout.reset()?;

                    self.color_spec
                        .set_fg(Some(colors::HIGHLIGHTED_LINE_FG_COLOR))
                        .set_bg(Some(colors::HIGHLIGHTED_WORD_BG_COLOR))
                        .set_bold(true);
                    self.stdout.set_color(&self.color_spec)?;
                    self.stdout.write(
                        &line.string.as_bytes()
                            [highlighted_substring_range.start..highlighted_substring_range.end],
                    )?;
                    self.color_spec.clear();
                    self.stdout.reset()?;

                    last_end = highlighted_substring_range.end;

                    Ok(())
                });

        if result.is_err() {
            return result;
        }

        self.color_spec
            .set_fg(Some(colors::HIGHLIGHTED_LINE_FG_COLOR));
        self.stdout.set_color(&self.color_spec)?;
        self.stdout.write(&line.string.as_bytes()[last_end..])?;
        self.color_spec.clear();
        self.stdout.reset()?;

        self.stdout.write(b"\n")?;

        Ok(())
    }
}

// impl Writer {
//     pub fn paint_line(
//         &self,
//         string: &str,
//         words_indices: Vec<(usize, usize)>,
//     ) -> io::Result<String> {
//         for word_indices in words_indices {
//             let word = &string[word_indices.0..word_indices.1];
//             &self.paint_searched_word(word);
//         }

//         Ok(String::from(string))
//     }

//     fn paint_searched_word(&self, sub_string: &str) -> io::Result<String> {
//         &self.paint(
//             sub_string,
//             Some(colors::HIGHLIGHTED_LINE_FG_COLOR),
//             Some(colors::HIGHLIGHTED_WORD_BG_COLOR),
//         )
//     }

//     fn paint(
//         mut self,
//         sub_string: &str,
//         color_fg: Option<Color>,
//         color_bg: Option<Color>,
//     ) -> io::Result<String> {
//         self.color_spec.set_fg(color_fg);
//         self.color_spec.set_bg(color_bg);
//         self.buffer.set_color(&self.color_spec)?;
//         self.buffer.write(sub_string.as_bytes())?;
//         let result: Result<String, std::string::FromUtf8Error> =
//             String::from_utf8(self.buffer.into_inner());
//         Ok(result.unwrap())
//     }
// }
