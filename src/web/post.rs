use crate::config::MPD_ADDRESS;
use crate::web::{errors, response};
use mpd::{Client, State, Term, Query};
use mpd::error::Result;
use std::borrow::Cow;

fn add_song(filename: &str) -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    let mut query = Query::new();
    mpd.findadd(query.and(Term::File, Cow::from(filename)))?;
    Ok(response::ok_no_content())
}

fn remove_song(position_str: &str) -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    if let Ok(position) = position_str.parse::<u32>() {
        mpd.delete(position)?;
    };
    Ok(response::ok_no_content())
}

fn seek(time_str: &str) -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    if let Ok(where_to) = time_str.trim().parse::<f64>() {
        mpd.rewind(where_to)?;
    }
    Ok(response::ok_no_content())
}

fn previous_song() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    mpd.prev()?;
    Ok(response::ok_no_content())
}

fn pause() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    if mpd.status()?.state == State::Stop {
        mpd.play()?;
    } else {
        mpd.toggle_pause()?;
    }
    Ok(response::ok_no_content())
}

fn next_song() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    mpd.next()?;
    Ok(response::ok_no_content())
}

pub fn handle(path: &str, body: &str) -> Result<String> {
    match path {
        "/addsong" => add_song(body),
        "/removesong" => remove_song(body),
        "/seek" => seek(body),
        "/prev" => previous_song(),
        "/pause" => pause(),
        "/next" => next_song(),
        _ => Ok(response::error(errors::NOT_FOUND)),
    }
}

