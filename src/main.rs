use rust_clone_server::handle_connection;
use std::net::TcpListener;

fn main() {
    let server = TcpListener::bind("127.0.0.1:7878");

    match server {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        handle_connection(stream);
                    }
                    Err(err) => {
                        println!("The Error is =>>> {err}");
                    }
                }
            }
        }
        Err(err) => {
            println!("The Error is => {err}")
        }
    }
    println!("Hello, world!");
}
