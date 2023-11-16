use clap::Parser;
use std::env;
use std::fs;
use std::path::Path;
use colored::*;
use std::process;

/// Simple program to concatenate a string or file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {

    /// Use to print the given input, can be a string or file path
    #[arg(default_value_t = String::new())]
    input: String,

    /// Print given input in reverse
    #[arg(short, long, action)]
    reverse: bool,

    /// Print given input but l o n g
    #[arg(short, long, action)]
    spaced: bool,

    /// Change text color
    #[arg(short, long, default_value_t = String::new())]
    color: String,

    // /// Translate input to pig latin
    //#[arg(short, long, action)]
    //piglatin: bool
}

// reverse characters in given string
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

// get current directory
fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn main() {
    let args = Args::parse();

    let mut user_input;
    let mut dir = get_current_working_dir();
    dir.push('/');
    dir.push_str(&args.input);

    // terminate program if given string/file is null
    if args.input.is_empty(){
        process::exit(0x0100);
    }
    // check if given input is complete path
    if Path::new(&args.input).exists(){
        // save file contents to string
        user_input = fs::read_to_string(&args.input.to_string()).expect("No file given");
    } // then check if given input is in current dir
    else if Path::new(&dir).exists(){
        // save file contents to string
        user_input = fs::read_to_string(dir.to_string()).expect("No file given");
    } 
    else{
        // user gave string
        user_input = args.input;
    }
    // terminate program if given string/file is null
    if user_input.is_empty(){
        process::exit(0x0100);
    }
    if args.reverse{
        // reverse given string
        user_input = reverse(&user_input);
    }

    if args.spaced{
        // push spaces between string characters
        let mut result = String::new();
        for (i, c) in user_input.clone().chars().enumerate() {
            result.push(c);
            if (i + 1) % 1 == 0 {
                result.push(' ');
            }
        }
        user_input = result;
    }

    // switch statement to print input in colors
    match args.color.to_lowercase().as_str(){
        "red"=>println!("{}", user_input.red()),
        "blue"=>println!("{}", user_input.blue()),
        "green"=>println!("{}", user_input.green()),
        "yellow"=>println!("{}", user_input.yellow()),
        "purple"=>println!("{}", user_input.purple()),
        "cyan"=>println!("{}", user_input.cyan()),
        "white"=>println!("{}", user_input.white()),
        "rainbow"=>rainbow_print(user_input),
        _=>println!("{}", user_input)
    }   
}

// prints input as rainbow by alternating colored prints on each character in input
fn rainbow_print(s: String){
    let char_vec: Vec<char> = s.chars().collect();
    let mut i = 0;
    for c in char_vec {
        if i >= 6{
            i = 0;
        }
        match i{
            0=>print!("{}", c.to_string().red()),
            1=>print!("{}", c.to_string().yellow()),
            2=>print!("{}", c.to_string().green()),
            3=>print!("{}", c.to_string().blue()),
            4=>print!("{}", c.to_string().cyan()),
            5=>print!("{}", c.to_string().purple()),
            _=>print!("{}", c.to_string().black())
        }
        if c != ' '{
            i+=1;
        }
    }
    print!("\n");
}

// translates input string into pig latin
// fn pig_latin_translate(s: &str) -> String {

// }
