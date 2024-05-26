use std::fs;
use std::fs::File;
use std::io;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

const CAT_VERSION: &str = "v1.0.0";

const ALLOWED_SEP_CHARS: &'static [char] = &['<', '>', '*', '@', '#', '%', '=', '-', '+'];

pub fn display_file_content(args: &[String]) -> io::Result<()> {
    let single_line = args.contains(&"--single".to_string()) || args.contains(&"-s".to_string());
    let version = args.contains(&"--version".to_string()) || args.contains(&"-v".to_string());

    if version {
        println!("cat version {}", CAT_VERSION);
        return Ok(());
    }

    let usage =
        "\x1b[32mUsage\x1b[0m: swiss cat <filename> [--single|-s] [--sep-char=<char>|-c=<char>]";

    let filtered_chars: Vec<char> = ALLOWED_SEP_CHARS
        .iter()
        .filter(|&c| *c != '=')
        .cloned()
        .collect();

    if args.len() < 3 || args[1] != "cat" {
        println!("Please provide the required arguments. Check the cli usage below");
        eprintln!("{}", usage);
        println!("Allowed characters are: {:?}", filtered_chars);
        std::process::exit(0);
    }

    let mut sep_char = '+';
    for arg in args {
        if arg.starts_with("--sep-char=") || arg.starts_with("-c=") {
            let parts: Vec<&str> = arg.split('=').collect();
            if parts.len() == 2 {
                let ch = parts[1].chars().next().unwrap_or('+');
                if ALLOWED_SEP_CHARS.contains(&ch) {
                    sep_char = ch;
                } else {
                    eprintln!(
                        "\x1b[31mError\x1b[0m: Invalid separator character. Allowed characters are: {:?}",
                        filtered_chars
                    );
                    eprintln!("{}", usage);
                    std::process::exit(0);
                }
            }
        }
    }

    let file_path: &String = &args[2];
    let file: Result<File, io::Error> = File::open(file_path);

    match file {
        Ok(_) => print!(""),
        Err(e) => {
            println!("\x1b[31mError\x1b[0m: {}", e);
            std::process::exit(0);
        }
    }

    let file_content = fs::read_to_string(file_path).expect("Error occured while read_to_string");

    let ss: SyntaxSet = SyntaxSet::load_defaults_newlines();
    let ts: ThemeSet = ThemeSet::load_defaults();
    let syntax: &syntect::parsing::SyntaxReference = ss
        .find_syntax_for_file(file_path)
        .unwrap()
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    let theme: &syntect::highlighting::Theme = &ts.themes["base16-ocean.dark"];

    let mut highlighter = HighlightLines::new(syntax, theme);
    let sep_char_escaped = format!("\x1b[32m{}\x1b[0m", sep_char);
    for _ in 0..100 {
        print!("{}", sep_char_escaped);
    }
    println!("");
    for line in LinesWithEndings::from(&file_content) {
        let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &ss).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        if single_line {
            print!("{}", escaped.trim());
        } else {
            print!("{}", escaped);
        }
    }
    for _ in 0..100 {
        print!("{}", sep_char_escaped);
    }

    Ok(())
}
