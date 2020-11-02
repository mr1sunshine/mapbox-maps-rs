use css_colors::Color as CssColor;
use css_colors::{hsl, hsla, rgb, rgba, RGBA};
use raster_color::Color as RasterColor;
use scan_fmt::scan_fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) struct Color {
    color: RGBA,
}

impl Color {
    pub fn new(color: RGBA) -> Self {
        Self { color }
    }

    pub fn new_with_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        let color = rgba(r as u8, g as u8, b as u8, a);
        Color::new(color)
    }

    pub fn new_with_string(s: &str) -> Result<Color, &'static str> {
        let color = if let Ok((h, s, l)) = scan_fmt!(s, "hsl({d}, {d}%, {d}%)", i32, u8, u8) {
            hsl(h, s, l).to_rgba()
        } else if let Ok((h, s, l, a)) =
            scan_fmt!(s, "hsla({d}, {d}%, {d}%, {f})", i32, u8, u8, f32)
        {
            hsla(h, s, l, a).to_rgba()
        } else if let Ok((r, g, b)) = scan_fmt!(s, "rgb({d}, {d}, {d})", u8, u8, u8) {
            rgb(r, g, b).to_rgba()
        } else if let Ok((r, g, b, a)) = scan_fmt!(s, "rgba({d}, {d}, {d}, {f})", u8, u8, u8, f32) {
            rgba(r, g, b, a)
        } else if s.to_owned().starts_with('#') {
            let raster_color = RasterColor::hex(s).expect("Unable to parse hex color");
            rgba(
                raster_color.r,
                raster_color.g,
                raster_color.b,
                raster_color.a as f32 / 255.0,
            )
        } else {
            return Err("Unsupported color format");
        };

        Ok(Color::new(color))
    }

    pub fn red(&self) -> f32 {
        self.color.r.as_f32()
    }

    pub fn green(&self) -> f32 {
        self.color.g.as_f32()
    }

    pub fn blue(&self) -> f32 {
        self.color.b.as_f32()
    }

    pub fn alpha(&self) -> f32 {
        self.color.a.as_f32()
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Color")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Color::new_with_string(s).map_err(|_| de::Error::custom("Unsupported color format"))
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref YELLOW: Color = Color::new(rgba(255, 255, 0, 1.0));
    }

    #[test]
    fn color_yellow_rgb() {
        let data = r#""rgb(255, 255, 0)""#;

        let t: Color = serde_json::from_str(data).expect("Expected valid Color type");
        assert_eq!(t, *YELLOW);
    }

    #[test]
    fn color_yellow_rgba() {
        let data = r#""rgba(255, 255, 0, 1.0)""#;

        let t: Color = serde_json::from_str(data).expect("Expected valid Color type");
        assert_eq!(t, *YELLOW);
    }

    #[test]
    fn color_yellow_hex() {
        let data = "\"#FFFF00\"";

        let t: Color = serde_json::from_str(data).expect("Expected valid Color type");
        assert_eq!(t, *YELLOW);
    }
    // #[test]
    // fn color_yellow_hsl() {
    //     let data = r#""hsl(60, 100%, 50%)""#;

    //     let t: Color = serde_json::from_str(data).expect("Expected valid Color type");
    //     assert_eq!(t, *YELLOW);
    // }

    // #[test]
    // fn color_yellow_hsla() {
    //     let data = r#""hsla(60, 100%, 50%, 1.0)""#;

    //     let t: Color = serde_json::from_str(data).expect("Expected valid Color type");
    //     assert_eq!(t, *YELLOW);
    // }
}
