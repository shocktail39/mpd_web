use crate::config;
use mpd::{Client, Song, State, Status};

mod static_resources;
mod response;

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

fn response_update(current_song: &Option<Song>, queue: &Vec<Song>, status: &Status) -> String {
    let now_playing_string = if let Some(song) = current_song {
        &song_string(song)
    } else {
        "nothing"
    };

    let queue_strings = {
        let mut qs = vec![];
        for song in queue {
            qs.push(song_string(song));
        }
        qs
    };

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

    response::ok(&json::stringify(json::object! {
        now_playing: now_playing_string,
        queue: queue_strings,
        queue_pos: queue_pos,
        elapsed: current_song_elapsed_time,
        duration: current_song_duration
    }), "application/json")
}

fn response_all_songs(mut mpd: Client) -> mpd::error::Result<String> {
    // i would much prefer to just pass in the vec of songs instead of the whole mpd client,
    // but listall doesn't properly return metadata, only file names,
    // so we gotta get a bit wacky.
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

fn handle_get(head: &str) -> mpd::error::Result<String> {
    let path_split = head[4..].split_once(" ");
    match path_split {
        Some(("/", _)) => {
            Ok(response::ok(static_resources::CONTROL_PANEL, "text/html"))
        }
        Some(("/style.css", _)) => {
            Ok(response::ok(static_resources::STYLE, "text/css"))
        }
        Some(("/script.js", _)) => {
            Ok(response::ok(static_resources::SCRIPT, "text/javascript"))
        }
        Some(("/update", _)) => {
            let mut mpd = Client::connect(config::MPD_ADDRESS)?;
            Ok(response_update(&mpd.currentsong()?, &mpd.queue()?, &mpd.status()?))
        }
        Some(("/allsongs", _)) => {
            let mpd = Client::connect(config::MPD_ADDRESS)?;
            response_all_songs(mpd)
        }
        _ => {
            Ok(response::error("404 Not Found"))
        }
    }
}

fn handle_post(head: &str, body: &str) -> mpd::error::Result<String> {
    let path_split = head[5..].split_once(" ");
    match path_split {
        Some(("/addsong", _)) => {
            let mut mpd = Client::connect(config::MPD_ADDRESS)?;
            let mut query = mpd::Query::new();
            mpd.findadd(query.and(mpd::Term::File, std::borrow::Cow::from(body)))?;
            Ok(response::ok_no_content())
        }
        Some(("/seek", _)) => {
            let mut mpd = Client::connect(config::MPD_ADDRESS)?;
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
        Some(("/prev", _)) => {
            let mut mpd = Client::connect(config::MPD_ADDRESS)?;
            mpd.prev()?;
            // workaround for freeze on skip
            mpd.pause(true)?;
            mpd.play()?;
            Ok(response::ok_no_content())
        }
        Some(("/pause", _)) => {
            let mut mpd = Client::connect(config::MPD_ADDRESS)?;
            if mpd.status()?.state == State::Stop {
                mpd.play()?;
            } else {
                mpd.toggle_pause()?;
            }
            Ok(response::ok_no_content())
        }
        Some(("/next", _)) => {
            let mut mpd = Client::connect(config::MPD_ADDRESS)?;
            mpd.next()?;
            // workaround for freeze on skip
            mpd.pause(true)?;
            mpd.play()?;
            Ok(response::ok_no_content())
        }
        _ => {
            Ok(response::error("404 Not Found"))
        }
    }
}

pub fn handle_request(head: &str, body: &str) -> mpd::error::Result<String> {
    let method_split = head.split_once(" ");
    match method_split {
        Some(("GET", _)) => handle_get(head),
        Some(("POST", _)) => handle_post(head, body),
        _ => Ok(response::error("405 Method Not Allowed"))
    }
}

