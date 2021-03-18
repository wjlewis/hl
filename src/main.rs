mod common;
mod scheme;

use clap::{App, Arg};
use std::fs;
use std::io;
use std::process;

fn main() {
    let matches = App::new("hl")
        .arg(Arg::with_name("lexer").value_name("lexer"))
        .arg(
            Arg::with_name("filename")
                .value_name("filename")
                .required(false),
        )
        .get_matches();

    let lexer_name = matches.value_of("lexer");
    if lexer_name.is_none() {
        eprintln!("Usage: {} <lexer-name> [<input>]", "hl");
        process::exit(1);
    }
    let lexer_name = lexer_name.unwrap();

    match get_input(matches.value_of("filename")) {
        Ok(input) => {
            let lexer = match lexer_name {
                // Add additional lexers here:
                "scheme" => scheme::Lexer::from(input.as_str()),
                _ => {
                    eprintln!("Invalid lexer: {}", lexer_name);
                    process::exit(1);
                }
            };
            println!("{}", lexer);
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            process::exit(1);
        }
    }
}

// If a file has been provided, read it into a string; otherwise, read from
// stdin.
fn get_input(filename: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(filename) = filename {
        match fs::read_to_string(filename) {
            Ok(contents) => Ok(contents),
            Err(e) => Err(Box::new(e)),
        }
    } else {
        let mut line = String::new();
        loop {
            match io::stdin().read_line(&mut line) {
                Ok(n) => {
                    if n == 0 {
                        return Ok(line);
                    }
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }
    }
}
