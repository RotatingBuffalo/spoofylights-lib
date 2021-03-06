use core::fmt;
#[cfg(target_arch = "arm")]
use rpi_led_matrix::LedColor;
use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Pixel {
    pub fn new(yeah: Option<(u8, u8, u8)>) -> Pixel {
        if yeah.is_none() {
            return Pixel {
                r: 255,
                g: 255,
                b: 255,
            };
        } else {
            let tuple = yeah.unwrap();
            return Pixel {
                r: tuple.0,
                g: tuple.1,
                b: tuple.2,
            };
        }
    }
    #[cfg(target_arch = "arm")]
    pub fn to_led_color(&mut self) -> LedColor {
        return LedColor {
            red: self.r,
            green: self.g,
            blue: self.b,
        };
    }
}
impl Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rtrn = "#".to_owned();
        rtrn += &format!("{:02x}{:02x}{:02x};", self.r, self.g, self.b).to_owned();
        write!(f, "{}", rtrn)
    }
}
