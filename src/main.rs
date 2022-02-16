use spoofylightslib::frame::{algos::Algos, Frame};
use spoofylightslib::raymond::javasimulator::JavaSimulator;
use spoofylightslib::raymond::Raymond;
use std::{thread, time};

fn main() {
    let framebuffer = time::Duration::from_millis(1000 / 25);
    let mut f = Frame::new(Algos::hue_wave);
    let mut target = JavaSimulator::new();
    target.connect();
    for i in 0..3000 {
        target.send_frame(&mut f);
        thread::sleep(framebuffer);
        Frame::tick(&mut f, i * 3);
    }
    target.close();
}
