#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let rgb_color = RGB { r: 255, g: 255, b: 255 };

        let hex_color = convert::rgb_to_hex(&rgb_color);

        assert_eq!(hex_color, "#FFFFFF");
    }

    #[test]
    fn test_color_blending() {
        let color1 = RGB { r: 255, g: 0, b: 0 };
        let color2 = RGB { r: 0, g: 0, b: 255 };

        let blended_color = convert::blend_rgb_colors(&color1, &color2, 0.5);

        assert_eq!(blended_color, RGB { r: 128, g: 0, b: 128 });
    }

    #[test]
    fn test_color_conversion_with_env() {
        let env_setting = std::env::var("COLOR_MODE").unwrap_or_else(|_| "normal".to_string());

        if env_setting == "special" {
            let special_rgb_color = RGB { r: 64, g: 64, b: 64 };

            let special_hex_color = convert::special_rgb_to_hex(&special_rgb_color);

            assert_eq!(special_hex_color, "#404040");
        } else {
            let rgb_color = RGB { r: 255, g: 255, b: 255 };
            let hex_color = convert::rgb_to_hex(&rgb_color);
            assert_eq!(hex_color, "#FFFFFF");
        }
    }
}