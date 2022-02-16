#[cfg(target_arch = "arm")]
use crate::frame::Frame;
#[cfg(target_arch = "arm")]
use crate::raymond::Raymond;
#[cfg(target_arch = "arm")]
use rpi_led_matrix::{LedCanvas, LedMatrix, LedMatrixOptions};
#[cfg(target_arch = "arm")]
struct Hardware {
    matrix: LedMatrix,
    canvas: LedCanvas,
}
#[cfg(target_arch = "arm")]
impl Raymond for Hardware {
    fn connect(&mut self) {
        let options = LedMatrixOptions::new();
        options.set_hardware_mapping("adafruit-hat");
        self.matrix = LedMatrix::new(Some(options), None).unwrap();
        self.canvas = self.matrix.offscreen_canvas();
    }
    fn send_frame(&mut self, f: &Frame) {
        for x in 0..32 {
            for y in 0..32 {
                for y in 0..32 {
                    self.canvas
                        .set(x, y, &f.this[(y * 32 + x) as usize].to_led_color());
                    self.matrix.swap(self.canvas);
                }
            }
        }
    }
    fn close(&mut self) {
        self.canvas.clear();
        self.matrix.swap(self.canvas);
        drop(self.canvas);
        drop(self.matrix);
    }
}
