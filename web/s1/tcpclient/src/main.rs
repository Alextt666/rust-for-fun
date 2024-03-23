use std::net::TcpStream;
fn main() {
    let _stream = TcpStream::connect("localhost:2024").unwrap();
}
