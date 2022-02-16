use crate::frame::Frame;

pub mod hardware;
pub mod javasimulator;
pub trait Raymond {
    fn connect(&mut self);
    fn send_frame(&mut self, f: &Frame);
    fn close(&mut self);
}
