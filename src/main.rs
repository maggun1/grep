use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use clap::{Arg, ArgAction, Command};
use regex::Regex;

fn main() -> Result<(), Error>{
    let matches = Command::new("grep")
        .arg(Arg::new("pattern")
            .required(true)
            .index(1))

        .arg(Arg::new("filename")
            .required(true)
            .index(2))

        .arg(Arg::new("after")
            .short('A')
            .long("after")
            .num_args(1)
            .default_value("0")
            .conflicts_with_all(&["before", "context"]))

        .arg(Arg::new("before")
            .short('B')
            .long("before")
            .num_args(1)
            .default_value("0")
            .conflicts_with("context"))

        .arg(Arg::new("context")
            .short('C')
            .long("context")
            .num_args(1)
            .default_value("0"))

        .arg(Arg::new("count")
            .short('c')
            .long("count")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("ignore-case")
            .short('i')
            .long("ignore-case")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("invert")
            .short('v')
            .long("invert")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("fixed")
            .short('F')
            .long("fixed")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("line-num")
            .short('n')
            .long("line-num")
            .action(ArgAction::SetTrue))
        .get_matches();

    let filename = matches.get_one::<String>("filename").unwrap();
    let file = File::open(filename)?;
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let after_match = matches.get_one::<String>("after").and_then(|a| {a.parse::<usize>().ok()}).unwrap();
    let before_match = matches.get_one::<String>("before").and_then(|b| {b.parse::<usize>().ok()}).unwrap();
    let context_match = matches.get_one::<String>("context").and_then(|a| {a.parse::<usize>().ok()}).unwrap();

    let count = matches.get_flag("count");
    let ignore_case = matches.get_flag("ignore-case");
    let invert = matches.get_flag("invert");
    let fixed = matches.get_flag("fixed");
    let line_num = matches.get_flag("line-num");

    let mut match_count = 0;
    let mut to_print = vec![];

    for (i, line) in lines.iter().enumerate() {
        let content_to_check = if ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        let pattern_to_check = if ignore_case {
            pattern.to_lowercase()
        } else {
            pattern.clone()
        };

        let is_match = if fixed {
            content_to_check == pattern_to_check
        }
        else {
            let regex = Regex::new(pattern_to_check.as_str()).unwrap();
            regex.is_match(&content_to_check)
        };

        let final_match = if invert {
            !is_match
        } else {
            is_match
        };

        if final_match {
            match_count += 1;

            let start = if context_match > 0 || before_match > 0 {
                i.saturating_sub(context_match.max(before_match))
            } else {
                i
            };

            let end = if context_match > 0 || after_match > 0 {
                (i + context_match.max(after_match)).min(lines.len() - 1)
            } else {
                i
            };

            for j in start..=end {
                to_print.push(j);
            }
        }
    }

    to_print.sort_unstable();
    to_print.dedup();
    for i in to_print {
        if line_num {
            print!("{} ", i + 1);
        }
        println!("{}", lines[i]);
    }

    if count {
        println!("Matched: {}", match_count);
    }
    Ok(())
}