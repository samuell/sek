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

fn print_usage() {
    println!("Usage: sek <command>");
    println!("(Command can be one of 'colorize', 'col', 'sam')");
}

fn cmd_colorize() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let colored_line = colorize_dna(line);
        println!("{colored_line}")
    }
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

            if cols.len() >= 21 {
                let mut newcols = cols[0..=4].to_vec();

                // Add colored CIGAR string
                let cig = vec![colorize_cigar(cols[5].to_string())];
                let mut cig_str: Vec<&str> = cig.iter().map(|s| s.as_str()).collect();
                newcols.append(&mut cig_str);

                newcols.extend(cols[6..=8].to_vec());

                // Add colored sequence string
                let seq = vec![colorize_dna(cols[9].to_string())];
                let mut seq_str: Vec<&str> = seq.iter().map(|s| s.as_str()).collect();
                newcols.append(&mut seq_str);

                let restcols = cols[10..=20].to_vec();
                newcols.extend(restcols);

                let output = newcols.join("\t");
                println!("{output}")
            }
        }
    }
}

fn colorize_dna(line: String) -> String {
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

fn colorize_cigar(line: String) -> String {
    const M: &str = "\x1b[100m"; // Dark grey
    const D: &str = "\x1b[41m";
    const I: &str = "\x1b[42m";
    const S: &str = "\x1b[45m";
    const H: &str = "\x1b[44m";
    const X: &str = "\x1b[0m";

    let mut colored_line = String::new();
    let parts: Vec<&str> = line.split_inclusive(&['M', 'D', 'I', 'S', 'H']).collect();

    for part in parts {
        let colored_part = match part.chars().last() {
            Some('D') => format!("{D}{part}{X}"),
            Some('M') => format!("{M}{part}{X}"),
            Some('I') => format!("{I}{part}{X}"),
            Some('S') => format!("{S}{part}{X}"),
            Some('H') => format!("{H}{part}{X}"),
            _ => part.to_string(),
        };
        colored_line += &colored_part;
    }

    return colored_line;
}
