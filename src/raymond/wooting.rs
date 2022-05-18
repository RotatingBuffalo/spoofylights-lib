use crate::frame::pixel::Pixel;
use wooting_sdk::{rgb::RgbKeyboard, IntoMatrixRowColumn};
#[link(name = "wooting-rgb-sdk64", kind = "dylib")]
extern "C" {
    #[link_name = "wooting-rgb-sdk64.lib"]
    pub fn wooting_rgb_kbd_connected() -> bool;
    pub fn wooting_rgb_set_disconnected(calback: extern "C" fn());
    pub fn wooting_rgb_reset() -> bool;
    pub fn wooting_rgb_direct_set_key(row: u8, column: u8, red: u8, green: u8, blue: u8) -> bool;
    pub fn wooting_rgb_direct_reset_key(row: u8, column: u8) -> bool;
    pub fn wooting_rgb_array_update_keyboard() -> bool;
    pub fn wooting_rgb_array_auto_update(auto_update: bool);
    pub fn wooting_rgb_array_set_single(row: u8, column: u8, red: u8, green: u8, blue: u8) -> bool;
    pub fn wooting_rgb_array_set_full(colors_buffer: *const u8);
}

// i hate this library.
struct Pair {
    row: u8,
    column: u8,
}
impl IntoMatrixRowColumn for Pair {
    fn into_matrix_row_and_column(&self) -> (u8, u8) {
        return (self.row, self.column);
    }
}
pub fn draw_frame(color_arr: Vec<Vec<Pixel>>) {
    for (i, row) in color_arr.iter().enumerate() {
        for (j, key) in row.iter().enumerate() {
            let (r, g, b) = (key.r, key.g, key.b);

            //RgbKeyboard.direct_set_key(
            //    Pair {
            //        row: i as u8,
            //        column: j as u8,
            //    },
            //    r,
            //    g,
            //    b,
            //);
            unsafe {
                wooting_rgb_array_set_single(i as u8, j as u8, r, g, b);
            }
        }
    }
    unsafe {
        wooting_rgb_array_update_keyboard();
    }
    //RgbKeyboard.array_update();
}
