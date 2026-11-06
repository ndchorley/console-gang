use std::{io::{Read, Write}, net::TcpStream, thread};

use assertor::{EqualityAssertion, assert_that};
use console_gang::server::Server;

#[test]
fn an_invalid_add_player_request_cant_be_processed() {
    let port = 7000;
    let server = Server { port };
    let _ = thread::spawn(move || { server.start(); });

    let mut stream = TcpStream::connect(format!("localhost:{}", port)).unwrap();
    let _ = stream.write(&[1]);
    let mut buffer = [0];
    let _ = stream.read(&mut buffer);

    assert_that!(buffer).is_equal_to([255]);
}
