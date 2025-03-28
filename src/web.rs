mod get;
mod post;
mod response;
mod static_resources;


pub fn handle_request(head: &str, body: &str) -> mpd::error::Result<String> {
    let method_split = head.split_once(" ");
    match method_split {
        Some(("GET", _)) => get::handle(head),
        Some(("POST", _)) => post::handle(head, body),
        _ => Ok(response::error("405 Method Not Allowed"))
    }
}

