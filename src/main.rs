mod response_builder;
mod models;
mod assembler;
mod dto;
mod controllers;
mod services;

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::os::raw::c_int;
use std::str;
use winapi::shared::inaddr::IN_ADDR;
use winapi::shared::ws2def::{AF_INET, SOCKADDR, SOCKADDR_IN, SOCK_STREAM};
use winapi::um::winsock2::{accept, bind, closesocket, listen, recv, send, socket, WSACleanup, WSAStartup, INVALID_SOCKET, SOCKET_ERROR, WSADATA};
use crate::controllers::recipe_controller::{add_recipe, delete_recipe_by_id, get_all_recipes, get_recipe_by_id, update_recipe};
use crate::response_builder::create_http_success_response;

// Define the format of the handlers for the response
type Handler = fn(&str, Option<&str>, &str) -> String;

pub fn get_dispatcher() -> HashMap<(&'static str, &'static str), Handler> {
    let mut dispatcher: HashMap<(&str, &str), Handler> = HashMap::new();

    // Register routes with methods
    dispatcher.insert(("GET", "/RecipeService/GetRecipeById"), get_recipe_by_id);
    dispatcher.insert(("GET", "/RecipeService/GetAllRecipes"), get_all_recipes);
    dispatcher.insert(("PUT", "/RecipeService/UpdateRecipe"), update_recipe);
    dispatcher.insert(("POST", "/RecipeService/AddRecipe"), add_recipe);
    dispatcher.insert(("DELETE", "/RecipeService/DeleteRecipeById"), delete_recipe_by_id);

    // Insert a lambda for shutdown
    dispatcher.insert(("GET", "/ShutDown"), |_req_content_type, _query_params, _req_body| {
        create_http_success_response("Shutting down")
    });

    dispatcher
}

fn htons(host_short: u16) -> u16 {
    host_short.to_be()
}

fn main() {

    unsafe {
        // Initialize win socket
        let mut wsa_data: WSADATA = std::mem::zeroed();

        // If win socket startup failed
        if WSAStartup(0x202, &mut wsa_data) != 0 {
            panic!("Failed to initialize win socket");
        }

        // Create a socket
        let sock_fd = socket(AF_INET, SOCK_STREAM, 0);

        // If it is an invalid socket
        if sock_fd == INVALID_SOCKET {
            panic!("Failed to create socket");
        }

        // Bind the socket to an address and port
        let mut server_addr = SOCKADDR_IN {
            sin_family: AF_INET as u16,
            sin_port: htons(8080),
            sin_addr: IN_ADDR {
                S_un: std::mem::zeroed(),
            },
            sin_zero: [0; 8],
        };

        // Define the localhost
        let localhost: u32 = u32::from(Ipv4Addr::UNSPECIFIED);

        // Assign to the S_addr field within the union to localhost
        *server_addr.sin_addr.S_un.S_addr_mut() = localhost;

        // Bind the socket file descriptor to the server address
        let bind_result = bind(
            sock_fd,
            &server_addr as *const SOCKADDR_IN as *const SOCKADDR,
            std::mem::size_of::<SOCKADDR_IN>() as c_int,
        );

        // If the bind result is a socket error
        if bind_result == SOCKET_ERROR {
            let error_code = winapi::um::winsock2::WSAGetLastError();
            panic!("Failed to bind socket, error code: {}", error_code);
        }

        // Listen for incoming connections
        if listen(sock_fd, 10) == SOCKET_ERROR {
            panic!("Failed to listen on socket");
        }

        // Server started successfully
        println!("Server listening on 127.0.0.1:8080");

        // Accept incoming connections while server is running
        loop {
            let mut client_addr: SOCKADDR_IN = std::mem::zeroed();
            let mut addr_len: c_int = std::mem::size_of::<SOCKADDR_IN>() as c_int;
            let client_fd = accept(
                sock_fd,
                &mut client_addr as *mut SOCKADDR_IN as *mut SOCKADDR,
                &mut addr_len as *mut c_int,
            );

            // If the client file descriptor is invalid
            if client_fd == INVALID_SOCKET {
                eprintln!("Failed to accept connection");
                continue;
            }

            println!("Accepted new connection");

            // Handle HTTP Request
            let keep_running = handle_http_request(client_fd);

            // Exit loop
            if keep_running < 1 {
                break;
            }

        }

        // Server closed by the api
        println!("Close socket and cleanup");

        // Cleanup win socket and the socket file descriptor
        closesocket(sock_fd);
        WSACleanup();
    }
}

unsafe fn handle_http_request(client_fd: usize) -> i8 {

    // Define the buffer for the incoming message
    let mut buffer = [0u8; 1024];

    // Receive data from the client
    let bytes_read = recv(
        client_fd,
        buffer.as_mut_ptr() as *mut winapi::ctypes::c_char,
        buffer.len() as c_int,
        0,
    );

    // If recv failed
    if bytes_read == SOCKET_ERROR {
        let error_code = winapi::um::winsock2::WSAGetLastError();
        eprintln!("Failed to read from connection, error code: {}", error_code);
        return -1;
    }

    // Build the response
    let response = build_response_from_request(buffer, bytes_read);

    // Send the response in chunks to ensure it is fully sent
    let mut total_sent = 0;
    let response_bytes = response.as_bytes();
    while total_sent < response_bytes.len() {
        let bytes_sent = send(
            client_fd,
            response_bytes[total_sent..].as_ptr() as *const winapi::ctypes::c_char,
            (response_bytes.len() - total_sent) as c_int,
            0,
        );

        // If send failed
        if bytes_sent == SOCKET_ERROR {
            let error_code = winapi::um::winsock2::WSAGetLastError();
            eprintln!("Failed to send response, error code: {}", error_code);
            return -1;
        }

        total_sent += bytes_sent as usize;
    }

    println!("Response sent successfully");

    // Close the client socket
    closesocket(client_fd);

    if response.contains("Shutting down") {
        return -1;
    }

    // Continue loop
    1
}

fn build_response_from_request(buffer: [u8; 1024], bytes_read: c_int) -> String {
    // Parse the received request
    let request = str::from_utf8(&buffer[..bytes_read as usize]).unwrap_or("");
    println!("Received request:\n{}", request);

    // Parse request line
    let mut lines = request.lines();
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let url  = parts.next().unwrap_or("/");


    let (path, query_params) = if let Some((p, q)) = url.split_once('?') {
        (p, Some(q))
    } else {
        (url, None)
    };

    // Parse headers
    let mut headers = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            // End of headers
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    // Extract Content-Type
    let content_type = headers
        .get("Content-Type")
        .map(|s| s.as_str())
        .unwrap_or("text/plain");

    // Parse body
    let body = lines.collect::<Vec<&str>>().join("\n");

    println!(
        "Method: {}, Path: {}, Query-Parameters: {:?}, Content-Type: {}, Body: {}",
        method, path, query_params, content_type, body
    );

    // Get dispatcher
    let dispatcher = get_dispatcher();

    // Find and execute the corresponding handler
    let response = if let Some(handler) = dispatcher.get(&(method, path)) {
        handler(content_type, query_params, &body)
    } else {
        response_builder::create_http_not_found_response(Some("404 Not Found"))
    };

    response
}
