use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8082").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!")
    }
}
