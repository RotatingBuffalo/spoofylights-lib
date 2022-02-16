#[cfg(target_arch = "arm")]
use crate::frame::Frame;
#[cfg(target_arch = "arm")]
use crate::raymond::Raymond;
#[cfg(target_arch = "arm")]
use rpi_led_matrix::{LedCanvas, LedMatrix, LedMatrixOptions};
#[cfg(target_arch = "arm")]
pub struct Hardware {
    matrix: LedMatrix,
    canvas: LedCanvas,
}
#[cfg(target_arch = "arm")]
impl Hardware {
    pub fn new() -> Hardware {
        let mut options = LedMatrixOptions::new();
        options.set_hardware_mapping("adafruit-hat");
        let m = LedMatrix::new(Some(options), None).unwrap();
        let c = m.offscreen_canvas();
        return Hardware {
            matrix: m,
            canvas: c,
        };
    }
}
#[cfg(target_arch = "arm")]
impl Raymond for Hardware {
    fn connect(&mut self) {
        // this doesn't do anything
    }
    fn send_frame(&mut self, f: &mut Frame) {
        for x in 0..32 {
            for y in 0..32 {
                self.matrix.canvas.set(
                    x,
                    y,
                    &f.this[(x as usize, y as usize)].to_led_color().clone(),
                );
            }
        }
        self.canvas = self.matrix.swap(self.matrix.canvas);
    }
    fn close(&mut self) {
        self.canvas.clear();
        let c = &self.canvas;
        self.matrix.swap(self.canvas);
    }
}
