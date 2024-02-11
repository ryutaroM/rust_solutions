use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("ryutaroM <example@dot.com>")
        .about("Rust uniqr")
        .arg(
            Arg::with_name("IN_FILE")
                .help("Input file")
                .default_value("-")
                .multiple(false),
        )
        .arg(
            Arg::with_name("OUT_FILE")
                .help("Output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("count")
                .help("Show counts")
                .short("c")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: String::from(matches.value_of("IN_FILE").unwrap()),
        out_file: if let Some(o) = matches.value_of("OUT_FILE") {
            Some(String::from(o))
        } else {
            None
        },
        count: matches.is_present("count"),
    })
}

pub fn run(c: Config) -> MyResult<()> {
    let mut file = open(&c.in_file).map_err(|e| format!("{}: {}", c.in_file, e))?;
    let mut out_file: Box<dyn Write> = match &c.out_file {
        Some(out_file) => Box::new(File::create(out_file)?),
        _ => Box::new(std::io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if c.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previos = String::new();
    let mut cnt: u64 = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previos.trim_end() {
            print(cnt, &previos)?;
            previos = line.clone();
            cnt = 0;
        }

        cnt += 1;
        line.clear();
    }
	print(cnt, &previos)?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
