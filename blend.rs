use std::env;

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
    
    fn blend_normal(c1: Color, c2: Color) -> Color {
        Color {
            r: (c1.r as u16 + c2.r as u16) / 2 as u8,
            g: (c1.g as u16 + c2.g as u16) / 2 as u8,
            b: (c1.b as u16 + c2.b as u16) / 2 as u8,
        }
    }

    fn blend_multiply(c1: Color, c2: Color) -> Color {
        Color {
            r: ((c1.r as u16 * c2.r as u16) / 255) as u8,
            g: ((c1.g as u16 * c2.g as u16) / 255) as u8,
            b: ((c1.b as u16 * c2.b as u16) / 255) as u8,
        }
    }

    fn blend_screen(c1: Color, c2: Color) -> Color {
        Color {
            r: (255 - (((255 - c1.r as u16) * (255 - c2.r as u16)) / 255)) as u8,
            g: (255 - (((255 - c1.g as u16) * (255 - c2.g as u16)) / 255)) as u8,
            b: (255 - (((255 - c1.b as u16) * (255 - c2.b as u16)) / 255)) as u8,
        }
    }
    
    fn blend_overlay(c1: Color, c2: Color) -> Color {
        Color {
             r: if c1.r < 128 { (2 * c1.r as u16 * c2.r as u16 / 255) as u8 } else { (255 - 2 * (255 - c1.r as u16) * (255 - c2.r as u16) / 255) as u8 },
             g: if c1.g < 128 { (2 * c1.g as u16 * c2.g as u16 / 255) as u8 } else { (255 - 2 * (255 - c1.g as u16) * (255 - c2.g as u16) / 255) as u8 },
             b: if c1.b < 128 { (2 * c1.b as u16 * c2.b as u16 / 255) as u8 } else { (255 - 2 * (255 - c1.b as u16) * (255 - c2.b as u16) / 255) as u8 },
        }
    }
}

fn main() {
    let color1_r: u8 = env::var("COLOR1_R").unwrap_or("128".to_string()).parse().unwrap();
    let color1_g: u8 = env::var("COLOR1_G").unwrap_or("128".to_string()).parse().unwrap();
    let color1_b: u8 = env::var("COLOR1_B").unwrap_or("128".to_string()).parse().unwrap();

    let color2_r: u8 = env::var("COLOR2_R").unwrap_or("64".to_string()).parse().unwrap();
    let color2_g: u8 = env::var("COLOR2_G").unwrap_or("64".to_string()).parse().unwrap();
    let color2_b: u8 = env::var("COLOR2_B").unwrap_or("64".to_string()).parse().unwrap();

    let color1 = Color::new(color1_r, color1_g, color1_b);
    let color2 = Color::new(color2_r, color2_g, color2_b);

    let normal_blend = Color::blend_normal(color1, color2);
    let multiply_blend = Color::blend_multiply(color1, color2);
    let screen_blend = Color::blend_screen(color1, color2);
    let overlay_blend = Color::blend_overlay(color1, color2);
    
    println!("Normal Blend: {:?}", normal_blend);
    println!("Multiply Blend: {:?}", multiply_blend);
    println!("Screen Blend: {:?}", screen_blend);
    println!("Overlay Blend: {:?}", overlay_blend);
}