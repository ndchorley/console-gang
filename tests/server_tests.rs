use std::{io::{Read, Write}, net::TcpStream, thread};

use assertor::{EqualityAssertion, assert_that};
use console_gang::server::Server;

#[test]
fn an_invalid_add_player_request_cant_be_processed() {
    let port = 7000;
    server_is_started_on(port);

    let mut stream = TcpStream::connect(localhost_on(port)).unwrap();
    let bytes_to_send = &[1];
    let _ = stream.write(bytes_to_send);

    let mut received_bytes = [0];
    let _ = stream.read(&mut received_bytes);

    assert_that!(received_bytes).is_equal_to([255]);
}

fn server_is_started_on(port: u16) {
    let server = Server { port };
    let _ = thread::spawn(move || { server.start(); });
}

fn localhost_on(port: u16) -> String {
    format!("localhost:{}", port)
}
