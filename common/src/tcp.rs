use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub struct TCPStream {
    pub stream: TcpStream,
}
impl TCPStream {
    pub fn read(&mut self) -> io::Result<String> {
        let mut buffer = [0; 1024];
        self.stream.read(&mut buffer)?;
        let message = String::from_utf8_lossy(&buffer);
        Ok(message.to_string())
    }
    pub fn write(&mut self, message: String) -> io::Result<()> {
        let message_len: u32 = message.len() as u32;
        self.stream.write(&message_len.to_be_bytes())?;
        self.stream.write(message.as_bytes())?;
        //self.stream.shutdown(Shutdown::Write)?;
        Ok(())
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.stream.shutdown(Shutdown::Both).unwrap();
        Ok(())
    }
}