use std::env;

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err("Invalid hex format".into());
    }

    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid value for red")?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid value for green")?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid value for blue")?;

    Ok((r, g, b))
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let mut h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    if h < 0.0 {
        h += 360.0;
    }

    let l = (max + min) / 2.0;
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    (h, s * 100.0, l * 100.0)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let s = s / 100.0;
    let l = l / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r_prime, g_prime, b_prime) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r_prime + m) * 255.0).round() as u8,
        ((g_prime + m) * 255.0).round() as u8,
        ((b_prime + m) * 255.0).round() as u8,
    )
}

fn main() {
    let hex = rgb_to_hex(255, 0, 0);
    println!("RGB to HEX: {}", hex);

    let rgb_result = hex_to_rgb("#FF0000");
    match rgb_result {
        Ok(rgb) => println!("HEX to RGB: {:?}", rgb),
        Err(e) => println!("Error: {}", e),
    }

    let hsl = rgb_to_hsl(255, 0, 0);
    println!("RGB to HSL: {:?}", hsl);

    let rgb = hsl_to_rgb(0.0, 100.0, 50.0);
    println!("HSL to RGB: {:?}", rgb);
}