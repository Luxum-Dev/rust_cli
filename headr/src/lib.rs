use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Read;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author="Bastien", version="0.1.0")]
pub struct Config {
    #[arg(help="Input files", default_value="-")]
    files: Vec<String>,
    #[arg(short='n', long, required=false, conflicts_with="bytes", default_value="10")]
    lines: usize,
    #[arg(short, long, required=false)]
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let args = Config::parse();
    Ok(Config{
        files: args.files,
        lines: args.lines,
        bytes: args.bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_file = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Ok(mut file) => {
                if num_file > 1 {
                    println!("{} ==> {} <==", if file_num > 0 {"\n"} else {""}, filename);
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                }else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
            Err(e) => println!("{} {}", filename, e)
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
