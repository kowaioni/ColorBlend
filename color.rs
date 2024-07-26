use std::env;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
    hex: String,
    hsl: (f32, f32, f32),
}

impl Color {
    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        let hex = Color::rgb_to_hex(red, green, blue);
        let hsl = Color::rgb_to_hsl(red, green, blue);
        Color {
            red,
            green,
            blue,
            hex,
            hsl,
        }
    }

    fn rgb_to_hex(red: u8, green: u8, blue: u8) -> String {
        format!("#{:02X}{:02X}{:02X}", red, green, blue)
    }

    fn rgb_to_hsl(red: u8, green: u8, blue: u8) -> (f32, f32, f32) {
        let r = red as f32 / 255.0;
        let g = green as f32 / 255.0;
        let b = blue as f32 / 255.0;
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let l = (max + min) / 2.0;

        if max == min {
            (0.0, 0.0, l)
        } else {
            let d = max - min;
            let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
            let h = if max == r {
                (g - b) / d + if g < b { 6.0 } else { 0.0 }
            } else if max == g {
                (b - r) / d + 2.0
            } else {
                (r - g) / d + 4.0
            };

            ((h * 60.0).round(), s, l)
        }
    }

    fn display(&self) {
        println!("RGB: ({}, {}, {})", self.red, self.green, self.blue);
        println!("HEX: {}", self.hex);
        println!("HSL: ({:.0}, {:.0}%, {:.0}%)", self.hsl.0, self.hsl.1 * 100.0, self.hsl.2 * 100.0);
    }

    // New blend function
    fn blend(&self, other: &Color) -> Color {
        let new_red = (self.red as u16 + other.red as u16) / 2 as u8;
        let new_green = (self.green as u16 + other.green as u16) / 2 as u8;
        let new_blue = (self.blue as u16 + other.blue as u16) / 2 as u8;
        Color::from_rgb(new_red, new_green, new_blue)
    }
}

fn main() {
    let default_red: u8 = env::var("DEFAULT_RED").unwrap_or_else(|_| "0".to_string()).parse().expect("Invalid value for DEFAULT_RED");
    let default_green: u8 = env::var("DEFAULT_GREEN").unwrap_or_else(|_| "0".to_string()).parse().expect("Invalid value for DEFAULT_GREEN");
    let default_blue: u8 = env::var("DEFAULT_BLUE").unwrap_or_else(|_| "0".to_string()).parse().expect("Invalid value for DEFAULT_BLUE");

    let color = Color::from_rgb(default_red, default_green, default_blue);

    color.display();

    // Example usage of blend function - blending with a hardcoded color
    let other_color = Color::from_rgb(255, 0, 0); // A bright red for blending
    let blended_color = color.blend(&other_color);
    println!("After blending:");
    blended_color.display();
}