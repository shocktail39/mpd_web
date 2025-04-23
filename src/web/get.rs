use crate::config::MPD_ADDRESS;
use crate::web::{errors, mime_types, response, static_resources};
use mpd::{Client, Song, State};
use mpd::error::Result;

fn song_to_json(song: &Song) -> json::JsonValue {
    let file = song.file.as_str();
    let title = if let Some(t) = &song.title {
        t.as_str()
    } else {
        file
    };
    let artist = if let Some(a) = &song.artist {
        a.as_str()
    } else {
        "Other"
    };
    json::object! {
        file: file,
        title: title,
        artist: artist
    }
}

fn queue() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    let queue = mpd.queue()?;
    let queue_objects = {
        let mut qs = vec![];
        for song in queue {
            qs.push(song_to_json(&song));
        }
        qs
    };

    let status = mpd.status()?;
    let queue_pos = if let Some(pos) = status.song {
        pos.pos
    } else {
        0
    };

    Ok(response::ok(&json::stringify(json::object! {
        queue: queue_objects,
        queue_pos: queue_pos
    }), mime_types::JSON))
}

fn now_playing() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    let current_song = mpd.currentsong()?;
    let now_playing = current_song.map(|song| song_to_json(&song));

    let status = mpd.status()?;
    let current_song_elapsed_time = if let Some(duration) = status.elapsed {
        duration.as_secs()
    } else {
        0
    };

    let current_song_duration = if let Some(duration) = status.duration {
        duration.as_secs()
    } else {
        0
    };

    let is_playing = mpd.status()?.state == State::Play;

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
    let mut song_list = vec![];
    for song in song_file_names {
        let song_with_info = &mpd.lsinfo(song)?[0];
        let title = if let Some(t) = &song_with_info.title {
            t
        } else {
            "Other"
        };
        let artist = if let Some(a) = &song_with_info.artist {
            a
        } else {
            "Other"
        };
        let file = song_with_info.file.as_str();
        song_list.push(json::object! {
            file: file,
            title: title,
            artist: artist
        });
    }
    Ok(response::ok(&json::stringify(song_list), mime_types::JSON))
}

pub fn handle(head: &str) -> Result<String> {
    let path = head[4..].split_once(" ").map(|(left, _right)| left);
    match path {
        Some("/") => Ok(response::ok(static_resources::CONTROL_PANEL, mime_types::HTML)),
        Some("/style.css") => Ok(response::ok(static_resources::STYLE, mime_types::CSS)),
        Some("/script.js") => Ok(response::ok(static_resources::SCRIPT, mime_types::JAVASCRIPT)),
        Some("/prev.svg") => Ok(response::ok(static_resources::PREV_SVG, mime_types::SVG)),
        Some("/pause.svg") => Ok(response::ok(static_resources::PAUSE_SVG, mime_types::SVG)),
        Some("/play.svg") => Ok(response::ok(static_resources::PLAY_SVG, mime_types::SVG)),
        Some("/next.svg") => Ok(response::ok(static_resources::NEXT_SVG, mime_types::SVG)),
        Some("/queue") => queue(),
        Some("/nowplaying") => now_playing(),
        Some("/allsongs") => all_songs(),
        Some(_) => Ok(response::error(errors::NOT_FOUND)),
        None => Ok(response::error(errors::BAD_REQUEST))
    }
}

