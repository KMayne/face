use hex;
use regex::Regex;
use skia_safe::{canvas::Canvas, Paint, Rect as SkRect};

#[derive(Debug)]
pub struct ColourParseError {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArgbColour {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}
impl ArgbColour {
    pub fn from_hex(hex: &str) -> Result<ArgbColour, ColourParseError> {
        let re = Regex::new(r"#?([\dA-Za-z]{8}|[\dA-Za-z]{6}|[\dA-Za-z]{3})").unwrap();
        let captures = re.captures(hex);
        let digits = captures.and_then(|c| { c.get(1) }).map(|m| m.as_str());
        match digits {
            Some(digit_string) => match digit_string.len() {
                3 => {
                    fn parse_hex_digit(char: &char) -> Option<u8> {
                        char.to_digit(16).map(|d| d as u8)
                    }
                    let mut chars = digit_string.chars();
                    let r = parse_hex_digit(&chars.next().unwrap()).unwrap() * 17;
                    let g = parse_hex_digit(&chars.next().unwrap()).unwrap() * 17;
                    let b = parse_hex_digit(&chars.next().unwrap()).unwrap() * 17;
                    Ok(ArgbColour { a: 255, r, g, b })
                },
                6 => {
                    hex::decode(digit_string)
                        .map(|b| ArgbColour { a: 255, r: b[0], g: b[1], b: b[2] })
                        .map_err(|_| ColourParseError {})
                }
                8 => {
                    hex::decode(digit_string)
                        .map(|b| ArgbColour { a: b[0], r: b[1], g: b[2], b: b[3] })
                        .map_err(|_| ColourParseError {})
                }
                _ => Err(ColourParseError {})
            },
            None => Err(ColourParseError {})
        }
    }
    fn invert(&self) -> ArgbColour {
        ArgbColour {
            a: self.a,
            r: self.r ^ 0xFF,
            g: self.g ^ 0xFF,
            b: self.b ^ 0xFF,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub stroke_colour: ArgbColour,
    pub fill_colour: ArgbColour
}

impl Rect {
    fn to_skia_rect(&self) -> SkRect {
        SkRect::from_xywh(self.x, self.y, self.width, self.height)
    }

    pub fn scale(&self, scale_factor: f32) -> Rect {
        Rect {
            x: self.x * scale_factor,
            y: self.y * scale_factor,
            width: self.width * scale_factor,
            height: self.height * scale_factor,
            ..*self
        }
    }
}

pub fn draw_ui(canvas: &mut Canvas, rects: Vec<Rect>) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_stroke_width(1.0);
    for rect in &rects {
        let colour = rect.fill_colour;
        paint.set_argb(colour.a, colour.r, colour.g, colour.b);
        canvas.draw_rect(rect.to_skia_rect(), &paint);
    }
}

mod test {
    use crate::renderer::ArgbColour;

    #[test]
    pub fn test_hex_to_argb() {
        assert_eq!(ArgbColour::from_hex("#ffffff").unwrap(), ArgbColour { a: 255, r: 255, g: 255, b: 255 });
        assert_eq!(ArgbColour::from_hex("#000000").unwrap(), ArgbColour { a: 255, r: 0, g: 0, b: 0 });
        assert_eq!(ArgbColour::from_hex("#FF0000").unwrap(), ArgbColour { a: 255, r: 255, g: 0, b: 0 });
        assert_eq!(ArgbColour::from_hex("#00FF00").unwrap(), ArgbColour { a: 255, r: 0, g: 255, b: 0 });
        assert_eq!(ArgbColour::from_hex("#0000FF").unwrap(), ArgbColour { a: 255, r: 0, g: 0, b: 255 });
    }
}