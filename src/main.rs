use spoofylightslib::frame::{algos::Algos, Frame, JavaFmt};
use std::{io::Write, net::TcpStream};
use std::{thread, time};

fn main() {
    let framebuffer = time::Duration::from_millis(1000 / 25);
    let mut f = Frame::new(Algos::hue_fade);
    let mut stream = TcpStream::connect("127.0.0.1:12000").expect("Failed to connect.");
    Frame::tick(&mut f, 1);
    for i in 0..3000 {
        stream.write(Frame::jfmt(&f).as_bytes()).ok();
        thread::sleep(framebuffer);
        Frame::tick(&mut f, i * 3);
    }
    stream.write(b".").ok();
    stream.shutdown(std::net::Shutdown::Both).ok();
}
