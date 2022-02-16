#[cfg(target_arch = "arm")]
use rpi_led_matrix::{LedCanvas, LedMatrix, LedMatrixOptions};

use crate::frame::Frame;
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
            for y in 0..32 {            for y in 0..32 {

                self.canvas.set(x, y, f.this.as_row_major()[y * 32 + x]);
                self.canvas.swap(self.canvas);
            }
        }
    }
    fn close(&mut self) {
        self.canvas.clear();
        self.matrix.swap(canvas);
        drop(canvas);
        drop(matrix);
    }
}