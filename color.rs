use std::env;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
    hex_code: String,
    hsl_values: (f32, f32, f32),
}

impl Color {
    fn new_from_rgb(red: u8, green: u8, blue: u8) -> Self {
        let hex_code = Color::convert_rgb_to_hex(red, green, blue);
        let hsl_values = Color::convert_rgb_to_hsl(red, green, blue);
        Color {
            red,
            green,
            blue,
            hex_code,
            hsl_values,
        }
    }

    fn convert_rgb_to_hex(red: u8, green: u8, blue: u8) -> String {
        format!("#{:02X}{:02X}{:02X}", red, green, blue)
    }

    fn convert_rgb_to_hsl(red: u8, green: u8, blue: u8) -> (f32, f32, f32) {
        let red_normalized = red as f32 / 255.0;
        let green_normalized = green as f32 / 255.0;
        let blue_normalized = blue as f32 / 255.0;
        let max_value = red_normalized.max(green_normalized.max(blue_normalized));
        let min_value = red_normalized.min(green_normalized.min(blue_normalized));
        let lightness = (max_value + min_value) / 2.0;

        if max_value == min_value {
            (0.0, 0.0, lightness)
        } else {
            let delta = max_value - min_value;
            let saturation = if lightness > 0.5 { delta / (2.0 - max_value - min_value) } else { delta / (max_value + min_value) };
            let hue = if max_value == red_normalized {
                (green_normalized - blue_normalized) / delta + if green_normalized < blue_normalized { 6.0 } else { 0.0 }
            } else if max_value == green_normalized {
                (blue_normalized - red_normalized) / delta + 2.0
            } else {
                (red_normalized - green_normalized) / delta + 4.0
            };

            ((hue * 60.0).round(), saturation, lightness)
        }
    }

    fn show_color_details(&self) {
        println!("RGB: ({}, {}, {})", self.red, self.green, self.blue);
        println!("Hex Code: {}", self.hex_code);
        println!("HSL: ({:.0}, {:.0}%, {:.0}%)", self.hsl_values.0, self.hsl_values.1 * 100.0, self.hsl_values.2 * 100.0);
    }

    fn blend_with(&self, other_color: &Color) -> Color {
        let blended_red = (self.red as u16 + other_color.red as u16) / 2 as u8;
        let blended_green = (self.green as u16 + other_color.green as u16) / 2 as u8;
        let blended_blue = (self.blue as u16 + other_color.blue as u16) / 2 as u8;
        Color::new_from_rgb(blended_red, blended_green, blended_blue)
    }
}

fn main() {
    let default_red: u8 = env::var("DEFAULT_RED").unwrap_or_else(|_| "0".to_string()).parse().expect("Invalid value for DEFAULT_RED");
    let default_green: u8 = env::var("DEFAULT_GREEN").unwrap_or_else(|_| "0".to_string()).parse().expect("Invalid value for DEFAULT_GREEN");
    let default_blue: u8 = env::var("DEFAULT_BLUE").unwrap_or_else(|_| "0".to_string()).parse().expect("Invalid value for DEFAULT_BLUE");

    let base_color = Color::new_from_rgb(default_red, default_green, default_blue);

    base_color.show_color_details();

    let blend_partner_color = Color::new_from_rgb(255, 0, 0); 
    let resulting_blended_color = base_color.blend_with(&blend_partner_color);
    println!("After blending:");
    resulting_blended_color.show_color_details();
}