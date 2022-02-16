use std::{
    io::Write,
    net::{Shutdown, TcpStream},
};

use crate::frame::{Frame, JavaFmt};

use super::Raymond;
pub struct JavaSimulator {
    stream: TcpStream,
}
impl JavaSimulator {
    pub fn new() -> JavaSimulator {
        JavaSimulator {
            stream: (TcpStream::connect("127.0.0.1:12000").unwrap()),
        }
    }
}
impl Raymond for JavaSimulator {
    fn connect(&mut self) {
        if let Ok(_stream) = TcpStream::connect("127.0.0.1:12000") {
            println!("connected to listener.")
        } else {
            println!("failed to connect to listener.")
        }
    }
    fn send_frame(&mut self, f: &mut Frame) {
        self.stream.write(Frame::jfmt(&f).as_bytes()).ok();
    }
    fn close(&mut self) {
        self.stream.write(b",").ok();
        self.stream
            .shutdown(Shutdown::Both)
            .expect("Shutdown failed.")
    }
}
