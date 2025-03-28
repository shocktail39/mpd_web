pub fn ok(body: &str, mime_type: &str) -> String {
    let head = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", mime_type, body.len());
    let mut response = head;
    response.push_str(body);
    response
}

pub fn ok_no_content() -> String {
    String::from("HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
}

pub fn error(code: &str) -> String {
    let body = format!("<!DOCTYPE html>
<html>
    <head>
        <title>{code}</title>
    </head>
    <body>
        <h1>{code}</h1>
    </body>
</html>");
    let head = format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\nContent-Length:{}\r\nConnection: close\r\n\r\n", code, body.len());
    let mut response = head;
    response.push_str(&body);
    response
}

