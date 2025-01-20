fn create_http_response(status: &str, content_type: &str, body: &str) -> String {
    let status_line = format!("HTTP/1.1 {}", status);
    let headers = format!(
        "Content-Type: {}\r\nContent-Length: {}\r\nConnection: close",
        content_type,
        body.len()
    );
    format_response(status_line, headers, body.to_string())
}

/// Creates a http success response.
pub fn create_http_success_response(body: &str) -> String {
    create_http_response("200 OK", "application/json", body)
}

/// Creates a http not found response.
/// Use None to use the default message.
/// Use Some("") to add a custom message to the response.
pub fn create_http_not_found_response(body: Option<&str>) -> String {
    let body = body.unwrap_or("The requested resource was not found.");
    create_http_response("404 Not Found", "text/plain", body)
}

/// Creates a http bad request response.
pub fn create_http_bad_request_response(body: &str) -> String {
    create_http_response("400 Bad Request", "text/plain", body)
}

/// Creates a http internal server error response
pub fn create_http_internal_server_error_response(body: &str) -> String {
    create_http_response("500 Internal Server Error", "text/plain", body)
}

/// Create a http created response
pub fn create_http_created_response(id: &str, body: &str) -> String {
    let status_line = "HTTP/1.1 201 Created";
    let headers = format!(
        "Content-Type: text/plain\r\nId: {}\r\nConnection: close",
        id
    );
    format_response(status_line.to_string(), headers, body.to_string())
}

/// Creates a http unauthorized response.
/// Currently not used
pub fn _create_http_unauthorized_response(body: &str) -> String {
    create_http_response("401 Unauthorized", "text/plain", body)
}

/// Creates a http forbidden response.
/// Currently not used
pub fn _create_http_forbidden_response(body: &str) -> String {
    create_http_response("403 Forbidden", "text/plain", body)
}

/// Creates a http no content response
/// Currently not used
pub fn _create_http_no_content_response() -> String {
    create_http_response("204 No Content", "text/plain", "")
}

/// Formats the response to a valid HTTP response
fn format_response(status_line: String, response_headers: String, response_body: String) -> String {
    format!(
        "{}\r\n{}\r\n\n{}",
        status_line,
        response_headers,
        response_body
    )
}
