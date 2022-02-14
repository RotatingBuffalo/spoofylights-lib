use array2d::Array2D;
use pixel::Pixel;
use std::fmt;
use std::fmt::Formatter;
pub struct Frame {
    pub this: Array2D<Pixel>,
    pub algo: fn(f: &mut Frame, i: Option<i32>),
}
impl Frame {
    pub fn new(a: fn(f: &mut Frame, i: Option<i32>)) -> Frame {
        return Frame {
            this: Array2D::filled_with(Pixel::new(Some((255, 255, 255))), 32, 32),
            algo: a,
        };
    }
    pub fn tick(f: &mut Frame, num: i32) {
        (f.algo)(f, Some(num));
    }
}
pub trait JavaFmt {
    fn jfmt(f: &Frame) -> String;
}
impl JavaFmt for Frame {
    fn jfmt(f: &Frame) -> String {
        let mut out: String = "{".to_owned();
        let pixels = f.this.as_row_major();
        for p in pixels {
            out += &p.to_string().to_owned();
        }
        out += "}\n";
        return out;
    }
}
impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.this)
    }
}
pub mod algos;
pub mod pixel;
