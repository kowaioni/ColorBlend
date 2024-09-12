use crate::color::{Color, blend, convert_to_rgb};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_new() {
        let color = Color::new(255, 100, 50);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 100);
        assert_eq!(color.blue, 50);
    }

    #[test]
    fn color_to_rgb_string() {
        let color = Color::new(255, 100, 50);
        assert_eq!(color.to_rgb_string(), "rgb(255, 100, 50)");
    }

    #[test]
    fn color_blend() {
        let color1 = Color::new(255, 100, 50);
        let color2 = Color::new(0, 100, 200);
        let blended_color = blend(color1, color2);
        assert_eq!(blended_color.red, 127);
        assert_eq!(blended_color.green, 100);
        assert_eq!(blended_color.blue, 125);
    }

    #[test]
    fn convert_hex_to_color() {
        let hex_str = "#FF6432";
        let color = convert_to_rgb(hex_str).unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 100);
        assert_eq!(color.blue, 50);
    }

    #[test]
    fn convert_invalid_hex_to_color() {
        let hex_str = "#GGGGGG"; // invalid hex code
        assert!(convert_to_rgb(hex_str).is_err());
    }
}