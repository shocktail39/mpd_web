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

    const CL_HEADER: &str = "\r\nContent-Length: ";
    let cl_header_split = head.split_once(CL_HEADER);
    let length_split = cl_header_split.and_then(|(_, right_side)| right_side.split_once("\r\n"));
    let maybe_length = length_split.and_then(|(length_str, _)| length_str.parse::<usize>().ok());
    let body = maybe_length.and_then(|length| {
        let mut buf = vec![0u8; length];
        reader.read_exact(&mut buf).ok().and_then(|()| String::from_utf8(buf).ok())
    }).unwrap_or_default();
    (head, body)
}

fn main() {
    let listener = TcpListener::bind(config::WEB_ADDRESS).unwrap();
    for result in listener.incoming() {
        match result {
            Ok(mut stream) => {
                thread::spawn(move||{
                    let (head, body) = parse_request(&mut stream);
                    stream.write_all(web::handle_request(&head, &body).as_slice()).unwrap();
                });
            }
            Err(e) => {eprintln!("{e}");}
        }
    }
}

