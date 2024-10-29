#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup() {
        dotenv::dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        println!("Using API_KEY: {}", api_key);
    }

    #[test]
    fn test_blend_white_black() {
        setup();
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);
        let blended = blend(white, black);
        assert_eq!(blended, Color::new(127, 127, 127), "Blending white and black should yield medium gray");
    }

    #[test]
    fn test_blend_red_blue() {
        setup();
        let red = Color::new(255, 0, 0);
        let blue = Color::new(0, 0, 255);
        let blended = blend(red, blue);
        assert_eq!(blended, Color::new(127, 0, 127), "Blending red and blue should yield purple");
    }

    #[test]
    fn test_convert_rgb_to_hsl() {
        setup();
        let rgb_color = Color::new(255, 0, 0);
        let hsl_color = rgb_color.to_hsl();
        assert_eq!(hsl_color, HSLColor::new(0, 100, 50), "RGB red should convert to HSL with hue=0, saturation=100, lightness=50");
    }

    #[test]
    fn test_convert_hsl_to_rgb() {
        setup();
        let hsl_color = HSLColor::new(120, 100, 50);
        let rgb_color = hsl_color.to_rgb();
        assert_eq!(rgb_color, Color::new(0, 255, 0), "HSL green should convert back to RGB green");
    }
}