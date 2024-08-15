use std::env;

struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

impl RgbColor {
    fn new(r: u8, g: u8, b: u8) -> RgbColor {
        RgbColor { r, g, b }
    }

    fn lighten(&self, amount: f32) -> RgbColor {
        RgbColor {
            r: self.r.saturating_add((255.0 * amount) as u8),
            g: self.g.saturating_add((255.0 * amount) as u8),
            b: self.b.saturating_add((255.0 * amount) as u8),
        }
    }

    fn darken(&self, amount: f32) -> RgbColor {
        RgbColor {
            r: self.r.saturating_sub((255.0 * amount) as u8),
            g: self.g.saturating_sub((255.0 * amount) as u8),
            b: self.b.saturating_sub((255.0 * amount) as u8),
        }
    }
}

fn main() {
    let light_amount_str = env::var("LIGHT_AMOUNT").unwrap_or("0.1".to_string());
    let dark_amount_str = env::var("DARK_AMOUNT").unwrap_or("0.1".to_string());

    let light_amount: f32 = light_amount_str.parse().unwrap_or(0.1);
    let dark_amount: f32 = dark_amount_str.parse().unwrap_or(0.1);

    let color = RgbColor::new(120, 65, 75);

    let lightened_color = color.lighten(light_amount);
    println!("Lightened Color: #{:02X}{:02X}{:02X}", lightened_color.r, lightened_color.g, lightened_color.b);
    
    let darkened_color = color.darken(dark_amount);
    println!("Darkened Color: #{:02X}{:02X}{:02X}", darkened_color.r, darkened_color.g, darkened_color.b);
}