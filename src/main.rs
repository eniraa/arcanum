use std::{env, fs, process};

mod utils;
use utils::output::{Issue, IssueGenus};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "{}",
            Issue {
                genus: IssueGenus::Error,
                message: Some("invalid arguments, expected 1 argument".to_string()),
                location: None,
                help: Some(format!("the expected format is `{} [script]`", args[0])),
            }
        );
        process::exit(1);
    }

    let script = match fs::read_to_string(&args[1]) {
        Ok(script) => script,
        Err(_) => {
            eprintln!(
                "{}",
                Issue {
                    genus: IssueGenus::Error,
                    message: Some("no such file or directory".to_string()),
                    location: None,
                    help: None,
                }
            );
            process::exit(1);
        }
    };

    println!("{}", script);
}
