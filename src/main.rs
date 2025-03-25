extern crate mpd;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub mod config;
pub mod web;

fn parse_request(stream: &mut TcpStream) -> (String, String) {
    let mut reader = BufReader::new(stream);
    let mut head = String::new();
    while !head.ends_with("\r\n\r\n") {
       let mut next_line = String::new();
       reader.read_line(&mut next_line).unwrap();
       head.push_str(&next_line);
    }

    const CL_HEADER: &str = "Content-Length: ";
    let mut body = String::new();
    if let Some(cl_position) = head.find(CL_HEADER) {
        if let Some((length_str, _)) = head[cl_position + CL_HEADER.len()..].split_once("\r\n") {
            if let Ok(length) = length_str.trim().parse::<usize>() {
                let mut buf = vec![0u8; length];
                let _ = reader.read_exact(&mut buf);
                body = String::from_utf8(buf).unwrap();
            }
        }
    }
    (head, body)
}

fn main() {
    let listener = TcpListener::bind(config::WEB_ADDRESS).unwrap();
    for result in listener.incoming() {
        match result {
            Ok(mut stream) => {
                thread::spawn(move||{
                    let (head, body) = parse_request(&mut stream);
                    stream.write_all(web::handle_request(&head, &body).unwrap().as_bytes()).unwrap();
                });
            }
            Err(e) => {eprintln!("{e}");}
        }
    }
}

