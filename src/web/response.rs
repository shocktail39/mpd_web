pub fn ok(body: &[u8], mime_type: &str) -> Vec<u8> {
    let head = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", mime_type, body.len()).into_bytes();
    let mut response = head;
    response.extend_from_slice(body);
    response
}

pub fn ok_no_content() -> Vec<u8> {
    Vec::from(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
}

pub fn error(code: &str) -> Vec<u8> {
    let body = format!("<!DOCTYPE html>
<html>
    <head>
        <title>{code}</title>
    </head>
    <body>
        <h1>{code}</h1>
    </body>
</html>").into_bytes();
    let head = format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", code, body.len()).into_bytes();
    let mut response = head;
    response.extend_from_slice(&body);
    response
}

