use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

enum ArgNames {
    Files,
    NumberLines,
    NumberLinesNon,
}

fn string(arg: ArgNames) -> &'static str {
    match arg {
        ArgNames::Files => "files",
        ArgNames::NumberLines => "number",
        ArgNames::NumberLinesNon => "number_nonblank",
    }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                let mut num = 1;
                for l in reader.lines() {
                    if config.number_lines {
                        println!("{:6}\t{}", num, &l?);
                        num += 1;
                    } else if config.number_nonblank_lines {
                        let line_copy = &l?.clone();
                        if line_copy.is_empty() {
                            println!();
                        } else {
                            println!("{:6}\t{}", num, line_copy);
                            num += 1;
                        }
                    } else {
                        println!("{}", &l?);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("hoge<example@dot.com")
        .about("Rust cat")
        .arg(
            Arg::with_name(string(ArgNames::Files))
                .value_name("FILES")
                .help("Input file")
                // .required(true)
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name(string(ArgNames::NumberLines))
                .conflicts_with(string(ArgNames::NumberLinesNon))
                .short("n")
                .long("number")
                .help("set numbers each lines without blank")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(string(ArgNames::NumberLinesNon))
                .short("b")
                .long("number-nonblank")
                .help("set number each lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy(string(ArgNames::Files)).unwrap(),
        number_lines: matches.is_present(string(ArgNames::NumberLines)),
        number_nonblank_lines: matches.is_present(string(ArgNames::NumberLinesNon)),
    })
}
