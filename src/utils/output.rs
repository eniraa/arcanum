use std::{fmt, fs};

pub struct CodeError {
    pub message: Option<String>,
    pub location: Option<CodeErrorLocation>,
    pub note: Option<String>,
}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text: String = "\x1b[0m\x1b[31;1merror\x1b[0m".to_string();

        if let Some(ref message) = self.message {
            text.push_str(format!("\x1b[1m:\x1b[0m {}", message).as_str());
        }

        if let Some(ref location) = self.location {
            let padding = " ".repeat(location.line.to_string().len() + 1);
            let script = fs::read_to_string(&location.file).expect("no such file or directory");
            let error_script = script.lines().nth(location.line - 1).unwrap();

            // error location
            text.push_str(
                format!(
                    "\n{}\x1b[34;1m@->\x1b[0m {}:{}:{}..{}",
                    padding,
                    location.file,
                    location.line,
                    location.columns.start,
                    location.columns.end
                )
                .as_str(),
            );

            // padding line
            text.push_str(format!("\n{}\x1b[34;1m|\x1b[0m", padding).as_str());

            // problematic code
            text.push_str(
                format!("\n\x1b[34;1m{} |\x1b[0m {}", location.line, error_script).as_str(),
            );

            // underline problematic code
            text.push_str(
                format!(
                    "\n{}\x1b[34;1m|\x1b[0m \x1b[31;1m{}\x1b[0m",
                    padding,
                    "^".repeat(location.columns.end - location.columns.start)
                )
                .as_str(),
            );
        }

        if let Some(ref note) = self.note {
            text.push_str(
                format!(
                    "\n\x1b[35;1mfix\x1b[39m:\x1b[0m {}\x1b[0m",
                    note
                )
                .as_str(),
            );
        }

        write!(f, "{}", text)?;
        Ok(())
    }
}

pub struct CodeErrorLocation {
    pub file: String,
    pub line: usize,
    pub columns: std::ops::Range<usize>,
}
