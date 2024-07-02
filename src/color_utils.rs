use crate::{consts::colors, utils::split_word_by_substring_inclusive};
use std::io::{self, Write};
use termcolor::{Buffer, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_matched_line(
    highlighted_line: String,
    before_lines: Vec<String>,
    after_lines: Vec<String>,
    matched_word: &str,
    case_sensitive: bool,
    consider_substrings: bool,
) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for line in before_lines {
        print_additional_lines(&mut stdout, &line).unwrap();
    }

    print_highlighted_line(
        &mut stdout,
        &highlighted_line,
        matched_word,
        case_sensitive,
        consider_substrings,
    )
    .unwrap();

    for line in after_lines {
        print_additional_lines(&mut stdout, &line).unwrap();
    }
}

pub fn get_highlighted_line(
    line: &str,
    search_word: &str,
    case_sensitive: bool,
    consider_substrings: bool,
) -> io::Result<String> {
    let mut buffer = Buffer::ansi();
    let mut color_spec = ColorSpec::new();

    let words: Vec<&str> = line.split(" ").collect();

    for word in words {
        color_spec.set_fg(Some(colors::HIGHLIGHTED_LINE_FG_COLOR));

        if word == search_word {
            color_spec.set_bg(Some(colors::HIGHLIGHTED_WORD_BG_COLOR));
        }

        if case_sensitive && word.to_lowercase() == search_word.to_lowercase() {
            color_spec.set_bg(Some(colors::HIGHLIGHTED_WORD_BG_COLOR));
        }

        if consider_substrings && word.contains(search_word)
            || case_sensitive
                && consider_substrings
                && word.to_lowercase().contains(&search_word.to_lowercase())
        {
            let word_parts = split_word_by_substring_inclusive(word, search_word);

            for word_part in word_parts {
                color_spec.set_fg(Some(colors::HIGHLIGHTED_LINE_FG_COLOR));

                if word_part == search_word
                    || case_sensitive && word_part.to_lowercase() == search_word.to_lowercase()
                {
                    color_spec.set_bg(Some(colors::HIGHLIGHTED_WORD_BG_COLOR));
                }

                buffer.set_color(&color_spec).unwrap();
                buffer.write(word_part.as_bytes()).unwrap();
                buffer.reset().unwrap();
                color_spec.clear();
            }

            buffer.write(b" ").unwrap();
            continue;
        }

        buffer.set_color(&color_spec)?;
        buffer.write(word.as_bytes())?;
        buffer.reset()?;
        buffer.write(b" ")?;
        color_spec.clear();
    }
    
    buffer.write(b"\n")?;

    buffer.write(b"\n")?;

    let result = String::from_utf8(buffer.into_inner()).unwrap();

    Ok(result)
}

pub fn print_highlighted_line(
    stdout: &mut StandardStream,
    line: &str,
    search_word: &str,
    case_sensitive: bool,
    consider_substrings: bool,
) -> io::Result<()> {
    let result =
        get_highlighted_line(line, search_word, case_sensitive, consider_substrings).unwrap();
    stdout.write(result.as_bytes())?;
    Ok(())
}

pub fn print_additional_lines(stdout: &mut StandardStream, line: &str) -> io::Result<()> {
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(colors::ADDITIONAL_LINE_FG_COLOR));
    stdout.set_color(&color_spec)?;
    stdout.write(line.as_bytes())?;
    stdout.write(b"\n")?;
    Ok(())
}
