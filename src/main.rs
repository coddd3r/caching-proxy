use std::{
    collections::HashMap,
    // io::{BufRead, BufReader, Read, Write},
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    os::unix::net::SocketAddr,
};

// use rustls::RootCertStore;
// use std::sync::Arc;
// use std::old_io::net::addrinfo::get_host_addresses;
use dns_lookup::lookup_host;
fn main() {
    let port = std::env::args().nth(1).expect("PROVIDE A PORT");
    let origin = std::env::args().nth(2).expect("PROVIDE AN ORIGIN URL");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    let cache: HashMap<&str, &str> = HashMap::new();

    for stream in listener.incoming() {
        let stream = stream.expect("UNEXPECTED stream failure");
        // handle_connection(stream, origin.clone());
        simple_handler(stream, origin.clone());
    }
}

// fn handle_connection(mut stream: TcpStream, origin: String) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_text: Vec<_> = buf_reader
//         .lines()
//         .map(|l| l.unwrap())
//         .take_while(|l| !l.is_empty())
//         .collect();

//     let root_store = RootCertStore {
//         roots: webpki_roots::TLS_SERVER_ROOTS.into(),
//     };

//     let mut client_config = rustls::ClientConfig::builder()
//         .with_root_certificates(root_store)
//         .with_no_client_auth();

//     client_config.key_log = Arc::new(rustls::KeyLogFile::new());

//     let addr = format!("{}:443", origin);
//     let server_name = origin.try_into().unwrap();
//     let mut conn = rustls::ClientConnection::new(Arc::new(client_config), server_name)
//         .expect("fialed to establish a client connection ");

//     let mut sock = TcpStream::connect(addr);
// }

fn simple_handler(mut stream: TcpStream, origin: String) {
    let buf_reader = BufReader::new(&mut stream);
    let request_text: Vec<String> = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    let Some(req_type) = request_text[0].split(" ").into_iter().next() else {
        return;
    };

    
    match req_type {
        "GET" => {
            let body = reqwest::blocking::get("https://www.rust-lang.org").text().unwrap();
                
        }
        "POST" => {
            let client = reqwest::blocking::Client::new();
            let res = client.post(&origin).body()
        }
        _ => eprintln!("UNIDENTIFIED REQ TYPE, {req_type}"),
    }
}
