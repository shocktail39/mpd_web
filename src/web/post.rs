use crate::config::MPD_ADDRESS;
use crate::web::response;
use mpd::{Client, State, Term, Query};
use mpd::error::Result;
use std::borrow::Cow;

fn add_song(body: &str) -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    let mut query = Query::new();
    mpd.findadd(query.and(Term::File, Cow::from(body)))?;
    Ok(response::ok_no_content())
}

fn seek(body: &str) -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    if let Ok(where_to) = body.trim().parse::<f64>() {
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
    let path_split = head[5..].split_once(" ");
    match path_split {
        Some(("/addsong", _)) => add_song(body),
        Some(("/seek", _)) => seek(body),
        Some(("/prev", _)) => previous_song(),
        Some(("/pause", _)) => pause(),
        Some(("/next", _)) => next_song(),
        Some(_) => Ok(response::error("404 Not Found")),
        None => Ok(response::error("400 Bad Request"))
    }
}

