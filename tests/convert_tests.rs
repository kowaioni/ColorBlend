#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hex_conversion() {
        let white_rgb = RGB { r: 255, g: 255, b: 255 };

        let white_hex = convert::rgb_to_hex(&white_rgb);

        assert_eq!(white_hex, "#FFFFFF");
    }

    #[test]
    fn test_rgb_color_blending() {
        let red_rgb = RGB { r: 255, g: 0, b: 0 };
        let blue_rgb = RGB { r: 0, g: 0, b: 255 };

        let blended_purple_rgb = convert::blend_rgb_colors(&red_rgb, &blue_rgb, 0.5);

        assert_eq!(blended_purple_rgb, RGB { r: 128, g: 0, b: 128 });
    }

    #[test]
    fn test_hex_conversion_with_environment_setting() {
        let color_mode_setting = std::env::var("COLOR_MODE").unwrap_or_else(|_| "normal".to_string());

        if color_mode_setting == "special" {
            let special_gray_rgb = RGB { r: 64, g: 64, b: 64 };

            let special_gray_hex = convert::special_rgb_to_hex(&special_gray_rgb);

            assert_eq!(special_gray_hex, "#404040");
        } else {
            let default_white_rgb = RGB { r: 255, g: 255, b: 255 };
            let default_white_hex = convert::rgb_to_hex(&default_white_rgb);
            assert_eq!(default_white_hex, "#FFFFFF");
        }
    }
}