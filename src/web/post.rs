use crate::config::MPD_ADDRESS;
use crate::web::{errors, response};
use mpd::{Client, State, Term, Query};
use std::borrow::Cow;

fn add_song(filename: &str) -> String {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let mut query = Query::new();
    let Ok(()) = mpd.findadd(query.and(Term::File, Cow::from(filename))) else {
        return response::error(errors::INTERNAL);
    };
    response::ok_no_content()
}

fn remove_song(position_str: &str) -> String {
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

fn seek(time_str: &str) -> String {
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

fn previous_song() -> String {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let Ok(()) = mpd.prev() else {
        return response::error(errors::INTERNAL);
    };
    response::ok_no_content()
}

fn pause() -> String {
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

fn next_song() -> String {
    let Ok(mut mpd) = Client::connect(MPD_ADDRESS) else {
        return response::error(errors::INTERNAL);
    };
    let Ok(()) = mpd.next() else {
        return response::error(errors::INTERNAL);
    };
    response::ok_no_content()
}

pub fn handle(path: &str, body: &str) -> String {
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

