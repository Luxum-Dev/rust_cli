use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

#[derive(Parser)]
#[command(name = "catr")]
#[command(author = "Bastien")]
#[command(version = "0.1.0")]
#[command(about = "Rust cat")]
struct Args {
    #[arg(help = "Input file(s)", default_value = "-")]
    file: Vec<String>,
    #[arg(
        short,
        long,
        help = "Number lines",
        conflicts_with = "number_nonblank_lines"
    )]
    number_lines: bool,
    #[arg(short = 'b', long, help = "Number Nonblank Lines")]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();
    Ok(Config {
        file: args.file,
        number_lines: args.number_lines,
        number_nonblank_lines: args.number_nonblank_lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.file {
        let mut counter = 1;
        match open(&filename){
            Ok(file) => {
                for line_result in file.lines(){
                    let line = line_result?;
                    if config.number_lines {
                        println!("{} {}", counter, line);
                        counter += 1;
                    }
                    else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            println!("{} {}", counter, line);
                            counter += 1
                        }else {
                            println!();
                        }
                    }
                    else{
                        println!("{}", line);
                    }
                }
            }
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
