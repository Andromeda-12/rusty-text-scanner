use crate::consts::colors;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_highlighted(
    before_lines: Vec<String>,
    highlighted_line: String,
    after_lines: Vec<String>,
    highlighted_slice: &str,
) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for line in before_lines {
        print_additional_lines(&mut stdout, &line)?;
    }

    print_highlighted_slices(&mut stdout, &highlighted_line, highlighted_slice)?;

    for line in after_lines {
        print_additional_lines(&mut stdout, &line)?;
    }

    Ok(())
}

pub fn print_highlighted_slices(
    stdout: &mut StandardStream,
    line: &str,
    highlighted_slice: &str,
) -> io::Result<()> {
    let mut color_spec = ColorSpec::new();

    let lowercase_highlighted_slice = highlighted_slice.to_lowercase();

    let line_parts: Vec<&str> = line.split(&lowercase_highlighted_slice).collect();

    println!("test {:?} {}", line_parts, line_parts.len());

    for (index, line_part) in line_parts.iter().enumerate() {
        if index == line_parts.len() - 1 {
            write!(stdout, "{line_part}")?;
            continue;
        }

        write!(stdout, "{line_part}")?;
        color_spec.set_bg(Some(colors::YELLOW));
        stdout.set_color(&color_spec)?;
        write!(stdout, "{highlighted_slice}")?;
        stdout.reset()?;
    }

    write!(stdout, "{}", '\n')?;

    Ok(())
}

pub fn print_additional_lines(stdout: &mut StandardStream, line: &str) -> io::Result<()> {
    print_colored_line(stdout, line, colors::GRAY)
}

pub fn print_colored_line(stdout: &mut StandardStream, line: &str, color: Color) -> io::Result<()> {
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(color));
    stdout.set_color(&color_spec)?;
    writeln!(stdout, "{}", line)?;
    stdout.reset()?;
    Ok(())
}
