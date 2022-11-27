#![allow(non_snake_case)]
#![allow(unused_variables)]

mod server;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let ip = std::env::var("IP").unwrap();
    let port = std::env::var("PORT").unwrap().parse::<u32>().unwrap();
    let username = std::env::var("USER_NAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();

    let server_connection = server::server::Server::new(ip, port, username, password).connect();

    // server::server::Server::upload_file(
    //     server_connection,
    //     "connection-test.html",
    //     "Hello World from Rust",
    //     "/public_html",
    // );

    let file_content =
        server::server::Server::get_file(server_connection, "public_html", "index.html");

    println!("{}", file_content);
}
