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
}

fn main() {
    let default_red = env::var("DEFAULT_RED").unwrap_or("0".into()).parse().unwrap_or(0);
    let default_green = env::var("DEFAULT_GREEN").unwrap_or("0".into()).parse().unwrap_or(0);
    let default_blue = env::var("DEFAULT_BLUE").unwrap_or("0".into()).parse().unwrap_or(0);

    let color = Color::from_rgb(default_red, default_green, default_blue);

    color.display();
}