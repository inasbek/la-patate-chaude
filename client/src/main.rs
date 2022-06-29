#[path = "../../common/src/tcp.rs"] mod tcp;
mod message;
use std::net::TcpStream;

fn main() {

    let mut stream = tcp::TCPStream {
        stream: TcpStream::connect("localhost:7878").unwrap()
    };

    stream.write(serde_json::to_string(&message::Message::Hello).unwrap()).unwrap();
    println!("{}", stream.read().unwrap());

    stream.write(serde_json::to_string(&message::Message::Subscribe(message::Subscribe {
        name: "test".to_string()
    })).unwrap()).unwrap();
    println!("{}", stream.read().unwrap());
}