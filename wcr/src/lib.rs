use clap::{App, Arg};
use std::fs::File;
use std::{
    error::Error,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("ryutaroM <example@dot.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .default_value("-")
                .help("Input files(s) [default: - ]")
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .help("Show line count")
                .short("l")
                .long("lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .help("Show word count")
                .short("w")
                .long("words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .help("Show byte count")
                .short("c")
                .long("bytes")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .help("Show character count")
                .short("m")
                .long("chars")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .get_matches();

    let lines = matches.is_present("lines");
    let words = matches.is_present("words");
    let bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");
    let mut flags = vec![lines, words, bytes, chars];
    if flags.iter().all(|v| v == &false) {
        flags[0] = true;
        flags[1] = true;
        flags[2] = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: flags[0],
        words: flags[1],
        bytes: flags[2],
        chars: chars,
    })
}

pub fn run(c: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &c.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                if let Ok(count) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(count.num_lines, c.lines),
                        format_field(count.num_words, c.words),
                        format_field(count.num_bytes, c.bytes),
                        format_field(count.num_chars, c.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );

                    total_lines += count.num_lines;
                    total_words += count.num_words;
                    total_bytes += count.num_bytes;
                    total_chars += count.num_chars;
                }
            }
        }
    }

    if c.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, c.lines),
            format_field(total_words, c.words),
            format_field(total_bytes, c.bytes),
            format_field(total_chars, c.chars),
        )
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut lines = String::new();
    loop {
        let bytes = file.read_line(&mut lines)?;
        if bytes == 0 {
            break;
        }
        num_bytes += bytes;
        num_lines += 1;
        num_words += lines.split_whitespace().count();
        num_chars += lines.chars().count();
        lines.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
