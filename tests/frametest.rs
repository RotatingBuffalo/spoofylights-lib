use spoofylightslib;
use spoofylightslib::frame::algos::Algos;
use spoofylightslib::frame::Frame;
#[test]
fn hue_fade() {
    let mut f = Frame::new(Algos::hue_fade);
    println!("{:?}", f.this.get(0, 0).unwrap());
    for i in 0..255 * 3 {
        Frame::tick(&mut f, i);
        println!("{:?}", f.this.get(0, 0).unwrap());
    }
}
