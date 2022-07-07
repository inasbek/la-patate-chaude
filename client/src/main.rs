#[path = "../../common/src/tcp.rs"] mod tcp;
mod message;
mod challenge;

use std::net::TcpStream;
use message::{MD5HashCashInput,MD5HashCashOutput};


fn main() {

    let mut stream = tcp::TCPStream {
        stream: TcpStream::connect("localhost:7878").unwrap()
    };

    let mut input = MD5HashCashInput {
        complexity: 9,
        message: "hello".to_string(),
    };

    // let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    // println!("Random string: {}", string);
    

    // let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    // assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");


    
    stream.write(serde_json::to_string(&message::Message::Hello).unwrap()).unwrap();
    println!("{}", stream.read().unwrap());

    if let Ok(message) = stream.read() {
        println!("first read");
        //let data: message::Message  = serde_json::from_str(&message).unwrap();

        // Do things just like with any other Rust data structure.
        //println!("Please call {:?}", data);
        println!("{}", message);
    } else {
        println!("Error reading message");
    }

    stream.write(serde_json::to_string(&message::Message::Subscribe(message::Subscribe {
        name: "test".to_string()
    })).unwrap()).unwrap();
    println!("{}", stream.read().unwrap());

    if let Ok(message2) = stream.read() {
        println!("{}", message2);
    } else {
        println!("Error reading message");
    }

    loop {

    }
}