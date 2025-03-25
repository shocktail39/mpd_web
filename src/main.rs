extern crate mpd;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::thread;

pub mod config;
pub mod web;

fn main() {
    let listener = TcpListener::bind(config::WEB_ADDRESS).unwrap();
    for result in listener.incoming() {
        match result {
            Ok(mut stream) => {
                thread::spawn(move||{
                    let mut reader = BufReader::new(&mut stream);
                    let mut first_line = String::new();
                    reader.read_line(&mut first_line).unwrap();
                    stream.write_all(web::handle_get(&first_line).unwrap().as_bytes()).unwrap();
                });
            }
            Err(e) => {eprintln!("{e}");}
        }
    }
}

