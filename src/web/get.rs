use crate::config::MPD_ADDRESS;
use crate::web::{errors, mime_types, response, static_resources};
use mpd::{Client, Song, State};
use mpd::error::Result;

fn song_to_json(song: &Song) -> json::JsonValue {
    let title = song.title.clone().unwrap_or_else(|| song.file.clone());
    let artist = song.artist.clone().unwrap_or_else(|| "Other".to_string());
    json::object! {
        file: song.file.clone(),
        title: title,
        artist: artist
    }
}

fn queue() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    let song_queue = mpd.queue()?;
    let json_queue = song_queue.into_iter().map(|song| song_to_json(&song)).collect::<Vec<_>>();

    let status = mpd.status()?;
    let queue_pos = status.song.map_or(0, |queue_place| queue_place.pos);

    Ok(response::ok(&json::stringify(json::object! {
        queue: json_queue,
        queue_pos: queue_pos
    }), mime_types::JSON))
}

fn now_playing() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    let current_song = mpd.currentsong()?;
    let now_playing = current_song.map(|song| song_to_json(&song));

    let status = mpd.status()?;
    let current_song_elapsed_time = status.elapsed.map_or(0, |duration| duration.as_secs());
    let current_song_duration = status.duration.map_or(0, |duration| duration.as_secs());
    let is_playing = status.state == State::Play;

    Ok(response::ok(&json::stringify(json::object! {
        now_playing: now_playing,
        elapsed: current_song_elapsed_time,
        duration: current_song_duration,
        is_playing: is_playing
    }), mime_types::JSON))
}

fn all_songs() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    let song_file_names = mpd.listall()?;
    let song_list = song_file_names.into_iter().filter_map(|filename| {
        // listall only gives file names, not info such as title or artist,
        // so we gotta query each file individually.
        let maybe_song = mpd.lsinfo(filename).ok().map(|mut song_vec| song_vec.swap_remove(0));
        maybe_song.map(|song| song_to_json(&song))
    }).collect::<Vec<_>>();
    Ok(response::ok(&json::stringify(song_list), mime_types::JSON))
}

pub fn handle(path: &str) -> Result<String> {
    match path {
        "/" => Ok(response::ok(static_resources::CONTROL_PANEL, mime_types::HTML)),
        "/style.css" => Ok(response::ok(static_resources::STYLE, mime_types::CSS)),
        "/script.js" => Ok(response::ok(static_resources::SCRIPT, mime_types::JAVASCRIPT)),
        "/prev.svg" => Ok(response::ok(static_resources::PREV_SVG, mime_types::SVG)),
        "/pause.svg" => Ok(response::ok(static_resources::PAUSE_SVG, mime_types::SVG)),
        "/play.svg" => Ok(response::ok(static_resources::PLAY_SVG, mime_types::SVG)),
        "/next.svg" => Ok(response::ok(static_resources::NEXT_SVG, mime_types::SVG)),
        "/queue" => queue(),
        "/nowplaying" => now_playing(),
        "/allsongs" => all_songs(),
        _ => Ok(response::error(errors::NOT_FOUND)),
    }
}

