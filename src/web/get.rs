use crate::config::MPD_ADDRESS;
use crate::web::{response, static_resources};
use mpd::{Client, Song};
use mpd::error::Result;

fn song_string(song: &Song) -> String {
    let mut to_return = if let Some(title) = &song.title {
        title
    } else {
        &song.file
    }.clone();
    if let Some(artist) = &song.artist {
        to_return.push_str(" by ");
        to_return.push_str(artist);
    }
    to_return
}

fn response_update() -> Result<String> {
    let mut mpd = Client::connect(MPD_ADDRESS)?;
    let current_song = mpd.currentsong()?;
    let now_playing_string = if let Some(song) = current_song {
        &song_string(&song)
    } else {
        "nothing"
    };

    let queue = mpd.queue()?;
    let queue_strings = {
        let mut qs = vec![];
        for song in queue {
            qs.push(song_string(&song));
        }
        qs
    };

    let status = mpd.status()?;
    let queue_pos = if let Some(pos) = status.song {
        pos.pos
    } else {
        0
    };

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

    Ok(response::ok(&json::stringify(json::object! {
        now_playing: now_playing_string,
        queue: queue_strings,
        queue_pos: queue_pos,
        elapsed: current_song_elapsed_time,
        duration: current_song_duration
    }), "application/json"))
}

fn response_all_songs() -> Result<String> {
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
    Ok(response::ok(&json::stringify(song_list), "application/json"))
}

pub fn handle(head: &str) -> Result<String> {
    let path_split = head[4..].split_once(" ");
    match path_split {
        Some(("/", _)) => Ok(response::ok(static_resources::CONTROL_PANEL, "text/html")),
        Some(("/style.css", _)) => Ok(response::ok(static_resources::STYLE, "text/css")),
        Some(("/script.js", _)) => Ok(response::ok(static_resources::SCRIPT, "text/javascript")),
        Some(("/update", _)) => response_update(),
        Some(("/allsongs", _)) => response_all_songs(),
        Some(_) => Ok(response::error("404 Not Found")),
        None => Ok(response::error("400 Bad Request"))
    }
}

