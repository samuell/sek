use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args.get(1).map(String::as_str);

    match command {
        Some("col") | Some("colorize") => cmd_colorize(),
        Some("sam") => cmd_sam_view(),
        _ => print_usage(),
    }
}

fn cmd_colorize() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let colored_line = colorize(line);
        println!("{colored_line}")
    }
}

fn colorize(line: String) -> String {
    const R: &str = "\x1b[41m";
    const G: &str = "\x1b[42m";
    const Y: &str = "\x1b[43m";
    const B: &str = "\x1b[44m";
    const X: &str = "\x1b[0m";

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

    return colored_line;
}

fn cmd_sam_view() {
    let stdin = io::stdin();
    let sam_headers =
        "query_name\tflag\tref_name\tref_pos\tmap_q\tcigar\trnext\tpnext\ttlen\tseq\tqual";

    println!("{sam_headers}");
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.chars().nth(0) != Some('@') {
            let cols: Vec<&str> = l.split("\t").collect();
            let mut newcols = cols[0..=8].to_vec();

            let seq = vec![colorize(cols[9].to_string())];
            let mut seq_str: Vec<&str> = seq.iter().map(|s| s.as_str()).collect();
            newcols.append(&mut seq_str);
            let output = newcols.join("\t");
            println!("{output}")
        }
    }
}

fn print_usage() {
    println!("Usage: sek <command>");
    println!("(Command can be one of 'colorize', 'col', 'sam')");
}
