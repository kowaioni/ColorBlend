use palette::{Srgb, Blend};

fn main() {
    let color1 = Srgb::new(1.0, 0.0, 0.0); // Red
    let color2 = Srgb::new(0.0, 0.0, 1.0); // Blue
    let blended_color = color1.into_linear().blend(&color2.into_linear());
    println!("Blended color: {:?}", blended_color.into_format());
}