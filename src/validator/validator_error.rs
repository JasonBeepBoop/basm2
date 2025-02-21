use crate::*;
use colored::*;
use std::fmt;
use term_size::dimensions;

#[derive(Debug, Clone)]
pub struct MacroValidatorError<'a> {
    pub err_input: String,
    pub err_message: String,
    pub help: Option<String>,
    pub orig_input: String,
    pub orig_pos: std::ops::Range<usize>, // macro call
    pub mac: &'a MacroContent,
}

impl fmt::Display for MacroValidatorError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m_pos = if let Some((_, v)) = self.mac.args.first() {
            v.clone()
        } else {
            0..0
        };
        if self.orig_pos.start >= self.orig_input.len()
            || self.orig_pos.end > self.orig_input.len()
            || self.orig_pos.start >= self.orig_pos.end
        {
            return writeln!(
                f,
                "Error: Indices {}:{} are out of bounds or invalid.",
                self.orig_pos.start, self.orig_pos.end
            );
        }

        let lines: Vec<&str> = self.orig_input.lines().collect();
        print_err_and_line(
            f,
            0,
            "error",
            self.orig_input.to_string(),
            self.err_message.to_string(),
            &Some(String::from("")),
            self.mac.file.to_string(),
            self.orig_pos.clone(),
            lines,
        )?;
        write!(f, "{}", "╮".bright_red())?;
        print_err_and_line(
            f,
            9,
            "",
            self.err_input.to_string(),
            format!(
                " in expansion of macro \"{}\"",
                self.mac.name
            ),
            &self.help,
            self.mac.file.to_string(),
            m_pos,
            self.err_input.lines().collect(),
        )?;
        Ok(())
    }
}

pub fn print_err_and_line(
    f: &mut fmt::Formatter<'_>,
    indents: usize,
    title: &str,
    text: String,
    msg: String,
    help: &Option<String>,
    file: String,
    pos: std::ops::Range<usize>,
    lines: Vec<&str>,
) -> fmt::Result {
    let terminal_width = dimensions().map(|(w, _)| w).unwrap_or(80);
    for (line_number, line) in lines.iter().enumerate() {
        let line_start = text
            .lines()
            .take(line_number)
            .map(|l| l.len() + 1)
            .sum::<usize>();
        let line_end = line_start + line.len();

        if (line_start <= pos.start && pos.start < line_end)
            || (line_start <= pos.end && pos.end < line_end)
        {
            let error_start = if pos.start >= line_start {
                pos.start - line_start
            } else {
                0
            };
            let error_end = if pos.end < line_end {
                pos.end - line_start
            } else {
                line.len()
            };
            let start_spaces = " ".repeat(indents);
            if title == "error" {
                writeln!(
                    f,
                    "{start_spaces}{}: {}",
                    title.bright_red().underline(),
                    msg.bold()
                )?;
            } else if title.is_empty() {
                writeln!(f, "{}", msg.bold())?;
            } else {
                writeln!(
                    f,
                    "{start_spaces}{}: {}",
                    title.yellow().underline(),
                    msg.bold()
                )?;
            }
            let left_char = if help.is_some() { "├" } else { "╰" };
            writeln!(
                f,
                "{start_spaces}{}{} {}:{}:{}",
                left_char.bright_red(),
                "─".bright_red(),
                file.green(),
                line_number + 1,
                error_start
            )?;

            let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
            let left_char = if help.is_some() { "│" } else { " " };

            if line.len() + 9 > terminal_width {
                let start = error_start.saturating_sub(terminal_width / 2 - 7);
                let end = (error_end + terminal_width / 2 - 7).min(line.len());
                let truncated_line = &line[start..end];

                writeln!(
                    f,
                    "{start_spaces}{}{:^6} {} {}",
                    left_char.bright_red(),
                    (line_number + 1).to_string().blue(),
                    "│".blue(),
                    truncated_line.trim()
                )?;

                let spaces = " ".repeat(error_start - start + 9 - leading_spaces);
                let arrows = "^".repeat(error_end.saturating_sub(error_start));
                write!(
                    f,
                    "{start_spaces}{}{}{}",
                    left_char.bright_red(),
                    spaces,
                    arrows.bright_red()
                )?;
            } else {
                writeln!(
                    f,
                    "{start_spaces}{}{:^6} {} {}",
                    left_char.bright_red(),
                    (line_number + 1).to_string().blue(),
                    "│".blue(),
                    line.trim()
                )?;

                let spaces = " ".repeat(error_start + 9 - leading_spaces);
                let arrows = "^".repeat(error_end.saturating_sub(error_start));
                write!(
                    f,
                    "{start_spaces}{}{}{}",
                    left_char.bright_red(),
                    spaces,
                    arrows.bright_red()
                )?;
            }

            if let Some(s) = &help {
                write!(
                    f,
                    "\n{start_spaces}{}{} {}: {s}",
                    "╰".bright_red(),
                    ">".yellow(),
                    "note".yellow()
                )?;
            }
        }
    }
    Ok(())
}
