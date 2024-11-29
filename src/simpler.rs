use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use rustls::RootCertStore;
use std::sync::Arc;
fn main() {
    let port = std::env::args().nth(1).expect("PROVIDE A PORT");
    let mut origin = std::env::args().nth(2).expect("PROVIDE AN ORIGIN URL");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    let cache: HashMap<&str, &str> = HashMap::new();

    for stream in listener.incoming() {
        let stream = stream.expect("UNEXPECTED stream failure");
        handle_connection(stream, origin.clone());
    }
}

fn handle_connection(mut stream: TcpStream, origin: String) {
    let buf_reader = BufReader::new(&mut stream);
    let request_text: Vec<_> = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    let mut client_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    client_config.key_log = Arc::new(rustls::KeyLogFile::new());

    let addr = format!("{}:443", origin);
    let server_name = origin.try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(client_config), server_name)
        .expect("fialed to establish a client connection ");

    let mut sock = TcpStream::connect(addr);
}
