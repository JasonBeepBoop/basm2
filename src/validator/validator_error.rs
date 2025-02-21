use colored::*;
use std::fmt;
use term_size::dimensions;

#[derive(Debug, Clone)]
pub struct ValidatorError {
    pub err_input: String,
    pub err_message: String,
    pub help: Option<String>,
    pub err_file: String,
    pub err_pos: std::ops::Range<usize>,
    pub orig_input: String,
    pub orig_pos: std::ops::Range<usize>,
}

impl fmt::Display for ValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.err_pos.start >= self.err_input.len()
            || self.err_pos.end > self.err_input.len()
            || self.err_pos.start >= self.err_pos.end
        {
            return writeln!(
                f,
                "Error: Indices {}:{} are out of bounds or invalid.",
                self.err_pos.start, self.err_pos.end
            );
        }

        let lines: Vec<&str> = self.err_input.lines().collect();
        let terminal_width = dimensions().map(|(w, _)| w).unwrap_or(80);

        for (line_number, line) in lines.iter().enumerate() {
            let line_start = self
                .err_input
                .lines()
                .take(line_number)
                .map(|l| l.len() + 1)
                .sum::<usize>();
            let line_end = line_start + line.len();

            if (line_start <= self.err_pos.start && self.err_pos.start < line_end)
                || (line_start <= self.err_pos.end && self.err_pos.end < line_end)
            {
                let error_start = if self.err_pos.start >= line_start {
                    self.err_pos.start - line_start
                } else {
                    0
                };
                let error_end = if self.err_pos.end < line_end {
                    self.err_pos.end - line_start
                } else {
                    line.len()
                };

                writeln!(
                    f,
                    "{}: {}",
                    "error".bright_red().underline(),
                    self.err_message.bold()
                )?;

                let left_char = if self.help.is_some() { "├" } else { "╰" };
                writeln!(
                    f,
                    "{}{} {}:{}:{}",
                    left_char.bright_red(),
                    "─".bright_red(),
                    self.err_file.green(),
                    line_number + 1,
                    error_start
                )?;

                let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
                let left_char = if self.help.is_some() { "│" } else { " " };

                if line.len() + 9 > terminal_width {
                    let start = error_start.saturating_sub(terminal_width / 2 - 7);
                    let end = (error_end + terminal_width / 2 - 7).min(line.len());
                    let truncated_line = &line[start..end];

                    writeln!(
                        f,
                        "{}{:^6} {} {}",
                        left_char.bright_red(),
                        (line_number + 1).to_string().blue(),
                        "│".blue(),
                        truncated_line.trim()
                    )?;

                    let spaces = " ".repeat(error_start - start + 9 - leading_spaces);
                    let arrows = "^".repeat(error_end.saturating_sub(error_start));
                    write!(
                        f,
                        "{}{}{}",
                        left_char.bright_red(),
                        spaces,
                        arrows.bright_red()
                    )?;
                } else {
                    writeln!(
                        f,
                        "{}{:^6} {} {}",
                        left_char.bright_red(),
                        (line_number + 1).to_string().blue(),
                        "│".blue(),
                        line.trim()
                    )?;

                    let spaces = " ".repeat(error_start + 9 - leading_spaces);
                    let arrows = "^".repeat(error_end.saturating_sub(error_start));
                    write!(
                        f,
                        "{}{}{}",
                        left_char.bright_red(),
                        spaces,
                        arrows.bright_red()
                    )?;
                }

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
