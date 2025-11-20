use crate::config::MPD_ADDRESS;
use crate::web::{errors, mime_types, response, static_resources};
use mpd::{Client, Song, State};

fn song_to_json(song: Song) -> json::JsonValue {
    let title = song.title.unwrap_or_else(|| song.file.clone());
    let artist = song.artist.unwrap_or_else(|| "Other".to_string());
    json::object! {
        file: song.file,
        title: title,
        artist: artist
    }
}

fn queue() -> Vec<u8> {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let Ok(song_queue) = mpd.queue() else {
        return response::error(errors::INTERNAL);
    };
    let json_queue = song_queue.into_iter().map(song_to_json).collect::<Vec<_>>();

    let Ok(status) = mpd.status() else {
        return response::error(errors::INTERNAL);
    };
    let queue_pos = status.song.map_or(0, |queue_place| queue_place.pos);

    response::ok(&json::stringify(json::object! {
        queue: json_queue,
        queue_pos: queue_pos
    }).as_bytes(), mime_types::JSON)
}

fn now_playing() -> Vec<u8> {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let Ok(current_song) = mpd.currentsong() else {
        return response::error(errors::INTERNAL);
    };
    let now_playing = current_song.map(song_to_json);

    let Ok(status) = mpd.status() else {
        return response::error(errors::INTERNAL);
    };
    let current_song_elapsed_time = status.elapsed.map_or(0, |duration| duration.as_secs());
    let current_song_duration = status.duration.map_or(0, |duration| duration.as_secs());
    let is_playing = status.state == State::Play;

    response::ok(&json::stringify(json::object! {
        now_playing: now_playing,
        elapsed: current_song_elapsed_time,
        duration: current_song_duration,
        is_playing: is_playing
    }).as_bytes(), mime_types::JSON)
}

fn all_songs() -> Vec<u8> {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let Ok(song_file_names) = mpd.listall() else {
        return response::error(errors::INTERNAL);
    };
    let song_list = song_file_names.into_iter().filter_map(|filename| {
        // listall only gives file names, not info such as title or artist,
        // so we gotta query each file individually.
        let maybe_song = mpd.lsinfo(filename).ok().map(|mut song_vec| song_vec.swap_remove(0));
        maybe_song.map(song_to_json)
    }).collect::<Vec<_>>();
    response::ok(&json::stringify(song_list).as_bytes(), mime_types::JSON)
}

pub fn handle(path: &str) -> Vec<u8> {
    match path {
        "/" => response::ok(static_resources::CONTROL_PANEL, mime_types::HTML),
        "/style.css" => response::ok(static_resources::STYLE, mime_types::CSS),
        "/script.js" => response::ok(static_resources::SCRIPT, mime_types::JAVASCRIPT),
        "/prev.svg" => response::ok(static_resources::PREV_SVG, mime_types::SVG),
        "/pause.svg" => response::ok(static_resources::PAUSE_SVG, mime_types::SVG),
        "/play.svg" => response::ok(static_resources::PLAY_SVG, mime_types::SVG),
        "/next.svg" => response::ok(static_resources::NEXT_SVG, mime_types::SVG),
        "/queue" => queue(),
        "/nowplaying" => now_playing(),
        "/allsongs" => all_songs(),
        _ => response::error(errors::NOT_FOUND),
    }
}

