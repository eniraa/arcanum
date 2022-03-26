use std::{fmt, fs};

pub struct Issue {
    pub genus: IssueGenus,
    pub message: Option<String>,
    pub location: Option<Locus>,
    pub help: Option<String>,
}

#[allow(dead_code)]
pub enum IssueGenus {
    Error,
    Warning,
}

impl IssueGenus {
    fn as_str(&self) -> &'static str {
        match self {
            IssueGenus::Error => "error",
            IssueGenus::Warning => "warning",
        }
    }

    fn ansi_color(&self) -> &'static str {
        match self {
            IssueGenus::Error => "\x1b[31",
            IssueGenus::Warning => "\x1b[33",
        }
    }
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: add features from #1

        let mut text: String = format!(
            "\x1b[0m{};1m{}\x1b[0m",
            self.genus.ansi_color(),
            self.genus.as_str()
        );

        if let Some(ref message) = self.message {
            text.push_str(format!("\x1b[1m:\x1b[0m {}", message).as_str());
        }

        if let Some(ref location) = self.location {
            let script = location.get_script();

            let line = location.get_line();
            let columns = location.get_columns();

            let padding = " ".repeat((line + 1).to_string().len() + 1);
            let issue_script = script.lines().nth(line).unwrap();

            // error location
            text.push_str(
                format!(
                    "\n{}\x1b[36;1m@->\x1b[0m {}:{}:{}..{}",
                    padding, location.file, line, columns.start, columns.end
                )
                .as_str(),
            );

            // padding line
            text.push_str(format!("\n{}\x1b[36;1m|\x1b[0m", padding).as_str());

            // problematic code
            text.push_str(format!("\n\x1b[36;1m{} | \x1b[0m{}", line, issue_script).as_str());

            // underline problematic code
            text.push_str(
                format!(
                    "\n{}\x1b[36;1m|\x1b[0m {}{};1m{}\x1b[0m",
                    padding,
                    " ".repeat(columns.start),
                    self.genus.ansi_color(),
                    "^".repeat(columns.len())
                )
                .as_str(),
            );
        }

        if let Some(ref help) = self.help {
            text.push_str(format!("\n\x1b[35;1mhelp\x1b[39m:\x1b[0m {}\x1b[0m", help).as_str());
        }

        writeln!(f, "{}", text)?;
        Ok(())
    }
}

pub struct Locus {
    pub file: String,
    pub range: std::ops::Range<usize>,
}

impl Locus {
    pub fn get_script(&self) -> String {
        fs::read_to_string(&self.file).expect("no such file or directory")
    }

    pub fn get_line(&self) -> usize {
        self.get_script()[..self.range.start].matches('\n').count()
    }

    pub fn get_columns(&self) -> std::ops::Range<usize> {
        let line = self.get_line();

        if line == 0 {
            self.range.clone()
        } else {
            let row_start = self
                .get_script()
                .match_indices('\n')
                .nth(line - 1)
                .expect("no such line")
                .0;
            (self.range.start - row_start)..(self.range.end - row_start)
        }
    }
}
