use std::{io::Write, net::TcpListener};

pub struct Server {
    pub port: u16
}

impl Server {
    pub fn start(&self) {
        let address = format!("localhost:{}", self.port);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let _ = stream.write(&[255]);
        }
    }
}
