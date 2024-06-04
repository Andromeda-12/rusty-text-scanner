use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_colored_line(stdout: &mut StandardStream, line: &str, color: Color) -> io::Result<()> {
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(color));
    stdout.set_color(&color_spec)?;
    writeln!(stdout, "{}", line)?;
    stdout.reset()?;
    Ok(())
}
