use std::{
    io::Write,
    net::{Shutdown, TcpListener, TcpStream},
};

use crate::frame::{Frame, JavaFmt};
struct JavaSimulator {
    stream: TcpStream,
}
impl JavaSimulator {
    pub fn connect(&mut self) {
        if let Ok(stream) = TcpStream::connect("127.0.0.1:12000") {
            println!("connected to listener.")
        } else {
            println!("failed to connect to listener.")
        }
    }
    pub fn sendFrame(&mut self, f: Frame) {
        self.stream.write(Frame::jfmt(&f).as_bytes());
    }
    pub fn close(&mut self) {
        self.stream
            .shutdown(Shutdown::Both)
            .expect("Shutdown failed.")
    }
}
