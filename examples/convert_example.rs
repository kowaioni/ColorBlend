use palette::{FromColor, Srgb, LinSrgb, Hsv};

fn main() {
    let color = Srgb::new(0.8, 0.2, 0.4);
    let linear_color = LinSrgb::from_color(color);
    println!("Linear sRGB: {:?}", linear_color);
    let hsv_color = Hsv::from_color(color);
    println!("HSV: {:?}", hsv_color);
}