use std::convert::TryFrom;

use array2d::Array2D;

use super::{pixel::Pixel, Frame};
pub enum Algos {
    SolidBlue,
    Default,
    RGBWave,
    RGBFade,
    HueFade,
}
impl Algos {
    pub fn solid_blue(f: &mut Frame, _counter: Option<i32>) {
        f.this = Array2D::filled_with(Pixel::new(Some((0, 0, 255))), 32, 32);
    }
    pub fn default(f: &mut Frame, _counter: Option<i32>) {
        f.this = Array2D::filled_with(Pixel::new(Some((255, 255, 255))), 32, 32);
    }
    pub fn hue_fade(f: &mut Frame, counter: Option<i32>) {
        let (mut red, mut green, mut blue) = (0u8, 0u8, 0u8);
        let i = counter.unwrap();
        let mut target = i / 255;
        target = target % 3;
        match target {
            0 => {
                red = u8::try_from(i % 255).ok().unwrap();
                blue = 255 - red;
            }
            1 => {
                green = u8::try_from(i % 255).ok().unwrap();
                red = 255 - green;
            }
            2 => {
                blue = u8::try_from(i % 255).ok().unwrap();
                green = 255 - blue;
            }
            _ => {
                panic!("I don't know how modulus works and the progam work. algos.rs");
            }
        }
        let pixels = Array2D::filled_with(Pixel::new(Some((red, green, blue))), 32, 32);
        f.this.clone_from(&pixels);
    }
}
