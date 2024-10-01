use std::{io::{self, Write}, iter::Scan};

enum META_COMMAND {
    EXIT,
    UNKNOWN,
}
enum STATEMENT {
    SELECT,
    INSERT,
    UNKNOWN,
}
fn main() {
    println!("Welcome to sql");
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("failed");
        let str = str.trim();

        if str.len() == 0 {
            println!("Error reading input");
            break;
        }
        if str.starts_with(".") {
            match do_meta_command(&str) {
                META_COMMAND::EXIT => {
                    println!("bye bye");
                    break;
                }
                META_COMMAND::UNKNOWN => {
                    println!("unknown meta cmd");
                    break;
                }
            }
        } else {
            match process_command(&str) {
                STATEMENT::INSERT => {
                    println!("do insert");
                    let statement:Vec<&str>=str.split_ascii_whitespace().collect();
                    let insert_data = (statement.get(1),statement.get(2));   
                }
                STATEMENT::SELECT => {
                    println!("do select")
                }
                STATEMENT::UNKNOWN => {
                    println!("unknown cmd")
                }
            }
        }
    }
}

fn do_meta_command(input: &str) -> META_COMMAND {
    if input == ".exit" {
        META_COMMAND::EXIT
    } else {
        META_COMMAND::UNKNOWN
    }
}

fn process_command(input: &str) -> STATEMENT {
    if input.to_lowercase().starts_with("select") {
        STATEMENT::SELECT
    } else if input.to_lowercase().starts_with("insert") {
        STATEMENT::INSERT
    } else {
        STATEMENT::UNKNOWN
    }
}
