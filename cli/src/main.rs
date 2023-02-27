use std::{fmt::Display, fs::read_to_string};

use clap::{arg, command, value_parser, Arg};

fn main() {
    let matches = command!()
        .arg(
            arg!([FILE])
                .value_parser(value_parser!(String))
                .default_value("test.md"),
        )
        .arg(
            Arg::new("t")
                .long("transform")
                .value_name("t")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let file = matches
        .get_one::<String>("FILE")
        .expect("the markdown file you want to parse");

    let target = if matches.get_flag("t") {
        Target::HTML
    } else {
        Target::Tokens
    };

    let content = read_to_string(file).unwrap_or_else(|e| {
        println!("Problems loading the file contents from: \"{}\"!\n", &file);
        panic!("{:?}", e);
    });

    println!(
        "Parsing {} [{} chars, to {}]:\n",
        &file,
        &content.len(),
        target
    );
}
