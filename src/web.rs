mod get;
mod errors;
mod mime_types;
mod post;
mod response;
mod static_resources;

pub fn handle_request(head: &str, body: &str) -> mpd::error::Result<String> {
    let first_line_split = head.split_once("\r\n");
    let method_split = first_line_split.and_then(|(first_line, _headers)| first_line.split_once(" "));
    let method_and_path = method_split.and_then(|(method, path_and_version)| {
        let path_split = path_and_version.split_once(" ");
        path_split.map(|(path, _version)| (method, path))
    });
    match method_and_path {
        Some(("GET", path)) => get::handle(path),
        Some(("POST", path)) => post::handle(path, body),
        Some((_unused_method, _path)) => Ok(response::error(errors::METHOD_NOT_ALLOWED)),
        None => Ok(response::error(errors::BAD_REQUEST))
    }
}

