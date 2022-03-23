use std::{fmt, fs};

pub struct CodeError {
    pub message: Option<String>,
    pub location: Option<CodeErrorLocation>,
    pub note: Option<String>,
}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text: String = "\x1b[0m\x1b[31;1merror\x1b[0m".to_string();

        if let Some(ref message) = self.message {
            text.push_str(format!("\x1b[1m:\x1b[0m {}", message).as_str());
        }

        if let Some(ref location) = self.location {
            let script = location.get_script();

            let line = location.get_line();
            let columns = location.get_columns();

            let padding = " ".repeat((line + 1).to_string().len() + 1);
            let error_script = script.lines().nth(line).unwrap();

            // error location
            text.push_str(
                format!(
                    "\n{}\x1b[34;1m@->\x1b[0m {}:{}:{}..{}",
                    padding,
                    location.file,
                    line,
                    columns.start,
                    columns.end
                )
                .as_str(),
            );

            // padding line
            text.push_str(format!("\n{}\x1b[34;1m|\x1b[0m", padding).as_str());

            // problematic code
            text.push_str(format!("\n\x1b[34;1m{} |\x1b[0m {}", line, error_script).as_str());

            // underline problematic code
            text.push_str(
                format!(
                    "\n{}\x1b[34;1m|\x1b[0m {}\x1b[31;1m{}\x1b[0m",
                    padding,
                    " ".repeat(columns.start),
                    "^".repeat(columns.len())
                )
                .as_str(),
            );
        }

        if let Some(ref note) = self.note {
            text.push_str(format!("\n\x1b[35;1mfix\x1b[39m:\x1b[0m {}\x1b[0m", note).as_str());
        }

        write!(f, "{}", text)?;
        Ok(())
    }
}

pub struct CodeErrorLocation {
    pub file: String,
    pub locus: std::ops::Range<usize>,
}

impl CodeErrorLocation {
    pub fn get_script(&self) -> String {
        fs::read_to_string(&self.file).expect("no such file or directory")
    }

    pub fn get_line(&self) -> usize {
        self.get_script()[..self.locus.start].matches('\n').count()
    }

    pub fn get_columns(&self) -> std::ops::Range<usize> {
        let line = self.get_line();

        if line == 0 {
            self.locus.clone()
        } else {
            let row_start = self
                .get_script()
                .match_indices('\n')
                .nth(line - 1)
                .expect("no such line")
                .0;
            (self.locus.start - row_start)..(self.locus.end - row_start)
        }
    }
}
