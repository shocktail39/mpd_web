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
    // workaround for freeze on skip
    if mpd.status()?.state == State::Play {
        mpd.pause(true)?;
        mpd.play()?;
    }
    Ok(response::ok_no_content())
}

fn seek(time_str: &str) -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    if let Ok(where_to) = time_str.trim().parse::<f64>() {
        mpd.rewind(where_to)?;
    }
    // workaround for freeze on skip
    if mpd.status()?.state == State::Play {
        mpd.pause(true)?;
        mpd.play()?;
    }
    Ok(response::ok_no_content())
}

fn previous_song() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    mpd.prev()?;
    // workaround for freeze on skip
    mpd.pause(true)?;
    mpd.play()?;
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
    // workaround for freeze on skip
    mpd.pause(true)?;
    mpd.play()?;
    Ok(response::ok_no_content())
}

pub fn handle(head: &str, body: &str) -> Result<String> {
    let path = head[5..].split_once(" ").map(|(left, _right)| left);
    match path {
        Some("/addsong") => add_song(body),
        Some("/removesong") => remove_song(body),
        Some("/seek") => seek(body),
        Some("/prev") => previous_song(),
        Some("/pause") => pause(),
        Some("/next") => next_song(),
        Some(_) => Ok(response::error(errors::NOT_FOUND)),
        None => Ok(response::error(errors::BAD_REQUEST))
    }
}

