use crate::frame::Frame;

pub mod hardware;
pub mod javasimulator;
pub mod wooting;
pub trait Raymond {
    fn connect(&mut self);
    fn send_frame(&mut self, f: &mut Frame);
    fn close(&mut self);
}
