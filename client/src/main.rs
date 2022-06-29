#[path = "../../common/src/tcp.rs"] mod tcp;

use std::net::TcpStream;
use serde::{Serialize, Deserialize};


struct SubscribeError {
    err : String
}

enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
}

fn main() {

    let mut stream = tcp::TCPStream {
        stream: TcpStream::connect("localhost:7878").unwrap()
    };

    stream.write(serde_json::to_string(&Message::Hello).unwrap()).unwrap();
    println!("{}", stream.read().unwrap());

    stream.write(serde_json::to_string(&Message::Subscribe(Subscribe {
        name: "test".to_string()
    })).unwrap()).unwrap();
    println!("{}", stream.read().unwrap());
}