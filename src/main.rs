use spoofylightslib::frame::{algos::Algos, Frame};
#[cfg(target_arch = "arm")]
use spoofylightslib::raymond::hardware::Hardware;
use spoofylightslib::raymond::javasimulator::JavaSimulator;
use spoofylightslib::raymond::Raymond;
use std::{thread, time};

fn main() {
    #[cfg(target_arch = "arm")]
    {
        let framebuffer = time::Duration::from_millis(1000 / 25);
        let mut f = Frame::new(Algos::hue_wave);
        let mut target = Hardware::new();
        target.connect();
        for i in 0..3000 {
            target.send_frame(&mut f);
            thread::sleep(framebuffer);
            Frame::tick(&mut f, i * 3);
        }
        target.close();
    }
    #[cfg(target_arch = "x86_64")]
    {}
}
