use colored::*;
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    pub input: String,
    pub message: String,
    pub help: Option<String>,
    pub file: String,
    pub start_pos: usize, // Start index of the error in the input string
    pub last_pos: usize,  // End index of the error in the input string
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start_pos >= self.input.len()
            || self.last_pos > self.input.len()
            || self.start_pos >= self.last_pos
        {
            return writeln!(
                f,
                "Error: Indices {}:{} are out of bounds or invalid.",
                self.start_pos, self.last_pos
            );
        }
        let lines: Vec<&str> = self.input.lines().collect();
        for (line_number, line) in lines.iter().enumerate() {
            let line_start = self
                .input
                .lines()
                .take(line_number)
                .map(|l| l.len() + 1)
                .sum::<usize>();
            let line_end = line_start + line.len();

            if (line_start <= self.start_pos && self.start_pos < line_end)
                || (line_start <= self.last_pos && self.last_pos < line_end)
            {
                let error_start = if self.start_pos >= line_start {
                    self.start_pos - line_start
                } else {
                    0
                };
                let error_end = if self.last_pos < line_end {
                    self.last_pos - line_start
                } else {
                    line.len()
                };
                writeln!(
                    f,
                    "{}: {}",
                    "error".bright_red().underline(),
                    self.message.bold()
                )?;
                let left_char = if self.help.is_some() { "├" } else { "╰" };
                writeln!(
                    f,
                    "{} {}:{}:{}",
                    left_char.bright_red(),
                    self.file.green(),
                    line_number + 1,
                    error_start
                )?;
                let left_char = if self.help.is_some() { "│" } else { " " };
                writeln!(
                    f,
                    "{}{:^6} {} {}",
                    left_char.bright_red(),
                    (line_number + 1).to_string().blue(),
                    "│".blue(),
                    line
                )?;
                let spaces = " ".repeat(error_start + 9);
                let arrows = "^".repeat(error_end.saturating_sub(error_start));
                write!(
                    f,
                    "{}{}{}",
                    left_char.bright_red(),
                    spaces,
                    arrows.bright_red()
                )?;
                if let Some(s) = &self.help {
                    writeln!(
                        f,
                        "\n{}{} {}: {s}",
                        "╰".bright_red(),
                        ">".yellow(),
                        "help".yellow()
                    )?;
                }
            }
        }

        Ok(())
    }
}
