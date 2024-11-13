use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream, UnixListener};
use std::{fs, thread};
use termion::{clear, color, cursor};

const FILE_PATH: &str = "/tmp/rst-log.sock";

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines().flatten() {
        match &line {
            s if s.starts_with("[ERR]") => {
                println!("{}{}", color::Fg(color::Red), line);
                print!("{}", color::Fg(color::Reset));
            },

            s if s.starts_with("[WRN]") => {
                println!("{}{}", color::Fg(color::LightYellow), line);
                print!("{}", color::Fg(color::Reset));
            },

            s if s.starts_with("[CLEAR]") => {
                print!("{}{}", clear::All, cursor::Goto(1, 1));
                // print_header();
            },

            _ => println!("{}", line),
        }
    }
}

fn print_header() {
    println!("Listening to incoming connections");
}

fn main() {
    if fs::metadata(FILE_PATH).is_ok() {
        match fs::remove_file(FILE_PATH) {
            Ok(_) => println!("Removed existing stream file"),
            Err(e) => {
                println!("Failed to remove existing stream file: {}", e);
                return;
            }
        }
    }

    println!("Starting log server");
    let listener = UnixListener::bind(FILE_PATH).unwrap();

    print_header();

    for stream in listener.incoming() {
        match stream {
            Ok(st) => {
                thread::spawn(|| handle_client(st));
            },
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
