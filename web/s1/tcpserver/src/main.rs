use std::net::TcpListener;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:2024").unwrap();
    println!("Running on port 2024...");
    for stream in listener.incoming(){
        let _stream = stream.unwrap();
        println!("Connection established!")
    }
}