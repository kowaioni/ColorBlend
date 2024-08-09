use std::env;
use std::num::ParseIntError;

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
            r: ((c1.r as u16 + c2.r as u16) / 2) as u8,
            g: ((c1.g as u16 + c2.g as u16) / 2) as u8,
            b: ((c1.b as u16 + c2.b as u16) / 2) as u8,
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

fn parse_color_component(value: Result<String, env::VarError>) -> Result<u8, ParseIntError> {
    value.unwrap_or("0".to_string()).parse::<u8>()
}

fn main() {
    let color1_r = parse_color_component(env::var("COLOR1_R")).expect("Failed to parse COLOR1_R");
    let color1_g = parse_color_component(env::var("COLOR1_G")).expect("Failed to parse COLOR1_G");
    let color1_b = parse_color_component(env::var("COLOR1_B")).expect("Failed to parse COLOR1_B");

    let color2_r = parse_color_component(env::var("COLOR2_R")).expect("Failed to parse COLOR2_R");
    let color2_g = parse_color_component(env::var("COLOR2_G")).expect("Failed to parse COLOR2_G");
    let color2_b = parse_color_component(env::var("COLOR2_B")).expect("Failed to parse COLOR2_B");

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