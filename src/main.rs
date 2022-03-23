use std::{env, fs, process};

mod utils;
use utils::output::CodeError;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "{}",
            CodeError {
                message: Some("invalid arguments, expected 1 argument".to_string()),
                location: None,
                note: Some(format!("the expected format is `{} [script]`", args[0])),
            }
        );
        process::exit(1);
    }

    let script = match fs::read_to_string(&args[1]) {
        Ok(script) => script,
        Err(_) => {
            eprintln!(
                "{}",
                CodeError {
                    message: Some("no such file or directory".to_string()),
                    location: None,
                    note: None,
                }
            );
            process::exit(1);
        }
    };

    println!("{}", script);
}
