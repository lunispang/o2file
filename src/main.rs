use std::io::{BufReader, Read};
use std::net::*;

const HTTP_OK: &str = "HTTP/1.1 200 OK\r\n";

fn http_ok(content: &[u8]) -> Vec<u8> {
    format!("{}Content-Length: {}\r\nContent-type: application/octet-stream\r\nContent-Disposition: attachment; filename=\"download\"\r\nConnection: close\r\n\r\n{}",
        HTTP_OK, 
        content.len(), 
        unsafe { String::from_utf8_unchecked(content.to_vec()) }
    ).bytes().collect()
}

fn stream_file(stream: &mut TcpStream, mut file: std::fs::File) -> Result<u64, std::io::Error> {
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    println!("{}", vec.len());
    let response = http_ok(&vec);
    let mut file: BufReader<_> = BufReader::new(response.as_slice());
    std::io::copy(&mut file, stream)
}

//const HTTP_NOT_FOUND: &[u8] = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Created TcpListener");

    let filename: &str = &std::env::args().skip(1).next().expect("a filename");

    for mut i in listener.incoming().filter_map(Result::ok) {
        println!("Accepted a client");
        let data = std::fs::File::open(filename).unwrap();
        let r = stream_file(&mut i, data);
        eprintln!("{r:?}");
    }
}
