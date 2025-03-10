use clap::Parser;
use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

/// Simple program to print a string or file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use to print the given input, can be a string or file path.
    #[arg(default_value_t = String::new())]
    input: String,

    /// Print given input in reverse.
    #[arg(short, long, action)]
    reverse: bool,

    /// Print given input but bubble spaced.
    #[arg(short, long, action)]
    spaced: bool,

    /// Change text color.
    #[arg(short, long, default_value_t = String::new())]
    color: String,
}

// Reverse characters in given string.
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

// Get current directory.
fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

// Main function.
fn main() {
    let args = Args::parse();

    let mut user_input;
    let mut dir = get_current_working_dir();
    dir.push('/');
    dir.push_str(&args.input);

    // Terminate program if given string/file is null.
    if args.input.is_empty() {
        process::exit(0x0100);
    }
    // Check if given input is complete path.
    if Path::new(&args.input).exists() {
        // save file contents to string
        user_input = fs::read_to_string(&args.input).expect("No file given");
    }
    // Check if given input is in current dir.
    else if Path::new(&dir).exists() {
        // save file contents to string
        user_input = fs::read_to_string(dir).expect("No file given");
    } else {
        // User gave input string.
        user_input = args.input;
        user_input.push('\n');
    }
    // Terminate program if given string/file is null.
    if user_input.is_empty() {
        process::exit(0x0100);
    }
    // Check if the string needs to be reversed.
    if args.reverse {
        // Reverse given string.
        user_input = reverse(&user_input);
    }
    // Check if the string needs to be spaced.
    if args.spaced {
        // Push spaces between string characters.
        let mut result = String::new();
        for c in user_input.clone().chars() {
            result.push(c);
            result.push(' ');
        }
        user_input = result;
    }

    // Switch statement to print input in colors.
    match args.color.to_lowercase().as_str() {
        "red" => println!("{}", user_input.red()),
        "blue" => println!("{}", user_input.blue()),
        "green" => println!("{}", user_input.green()),
        "yellow" => println!("{}", user_input.yellow()),
        "purple" => println!("{}", user_input.purple()),
        "cyan" => println!("{}", user_input.cyan()),
        "white" => println!("{}", user_input.white()),
        "rainbow" => rainbow_print(user_input),
        _ => println!("{}", user_input),
    }
}

// Prints input as rainbow by alternating colored prints on each character in input.
fn rainbow_print(s: String) {
    let char_vec: Vec<char> = s.chars().collect();
    let mut i = 0;
    for c in char_vec {
        if i >= 6 {
            i = 0;
        }
        match i {
            0 => print!("{}", c.to_string().red()),
            1 => print!("{}", c.to_string().yellow()),
            2 => print!("{}", c.to_string().green()),
            3 => print!("{}", c.to_string().blue()),
            4 => print!("{}", c.to_string().cyan()),
            5 => print!("{}", c.to_string().purple()),
            _ => print!("{}", c.to_string().black()),
        }
        if c != ' ' {
            i += 1;
        }
    }
}
