mod get;
mod errors;
mod mime_types;
mod post;
mod response;
mod static_resources;

pub fn handle_request(head: &str, body: &str) -> mpd::error::Result<String> {
    let method = head.split_once(" ").map(|(left, _right)| left);
    match method {
        Some("GET") => get::handle(head),
        Some("POST") => post::handle(head, body),
        Some(_) => Ok(response::error(errors::METHOD_NOT_ALLOWED)),
        None => Ok(response::error(errors::BAD_REQUEST))
    }
}

