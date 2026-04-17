use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: sek <command>");
        println!("(Command can be one of 'colorize')");
        return
    }
    let command = &args[1];
    println!("Got command: {command}");

    if command == "colorize" {
        const R: &str = "\x1b[31m";
        const G: &str = "\x1b[32m";
        const Y: &str = "\x1b[33m";
        const B: &str = "\x1b[34m";
        const X: &str = "\x1b[0m";

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Failed to read line");
            let mut colored_line = String::new();
            for c in line.chars() {
                let colored_c = match c {
                    cc @ ('A' | 'a') => format!("{G}{cc}{X}"),
                    cc @ ('C' | 'c') => format!("{Y}{cc}{X}"),
                    cc @ ('G' | 'g') => format!("{R}{cc}{X}"),
                    cc @ ('T' | 't') => format!("{B}{cc}{X}"),
                    _ => c.to_string(),
                };
                colored_line += &colored_c;
            }
            println!("{colored_line}")
        }
    }
}
