use crate::config::MPD_ADDRESS;
use crate::web::{errors, response};
use mpd::{Client, State};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn add_song(filename: &str) -> Vec<u8> {
    // the mpd crate doesn't have a function for the add command,
    // so let's make our own.
    let Ok(mut mpd) = TcpStream::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    {
        let mut buffer = String::new();
        let mut reader = BufReader::new(&mut mpd);
        let Ok(_bytes_read) = reader.read_line(&mut buffer) else {
            return response::error(errors::INTERNAL);
        };
        if !buffer.starts_with("OK MPD ") {
            return response::error(errors::INTERNAL);
        }
    }
    let escaped_filename = filename.replace("\"", "\\\"");
    let mpd_command = format!("add \"{escaped_filename}\"\n").into_bytes();
    let Ok(()) = mpd.write_all(&mpd_command) else {
        return response::error(errors::INTERNAL);
    };
    response::ok_no_content()
}

fn remove_song(position_str: &str) -> Vec<u8> {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    if let Ok(position) = position_str.parse::<u32>() {
        let Ok(()) = mpd.delete(position) else {
            return response::error(errors::INTERNAL);
        };
    };
    response::ok_no_content()
}

fn seek(time_str: &str) -> Vec<u8> {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    if let Ok(where_to) = time_str.trim().parse::<f64>() {
        let Ok(()) = mpd.rewind(where_to) else {
            return response::error(errors::INTERNAL);
        };
    }
    response::ok_no_content()
}

fn previous_song() -> Vec<u8> {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let Ok(()) = mpd.prev() else {
        return response::error(errors::INTERNAL);
    };
    response::ok_no_content()
}

fn pause() -> Vec<u8> {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let Ok(status) = mpd.status() else {
        return response::error(errors::INTERNAL);
    };
    if status.state == State::Stop {
        let Ok(()) = mpd.play() else {
            return response::error(errors::INTERNAL);
        };
    } else {
        let Ok(()) = mpd.toggle_pause() else {
            return response::error(errors::INTERNAL);
        };
    }
    response::ok_no_content()
}

fn next_song() -> Vec<u8> {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let Ok(()) = mpd.next() else {
        return response::error(errors::INTERNAL);
    };
    response::ok_no_content()
}

pub fn handle(path: &str, body: &str) -> Vec<u8> {
    match path {
        "/addsong" => add_song(body),
        "/removesong" => remove_song(body),
        "/seek" => seek(body),
        "/prev" => previous_song(),
        "/pause" => pause(),
        "/next" => next_song(),
        _ => response::error(errors::NOT_FOUND),
    }
}

